import FirebasePerformance
import Foundation

/// `native:perf` over Firebase Performance Monitoring (soar-plugin-firebase-perf).
public final class PerfModule: SoarModule {
    public let name = "perf"
    public init() { _ = SoarFirebase.configured }

    private enum Failure: Error { case invalid(String) }

    /// Running traces by name. Module functions may run on the bridge thread, so access is locked.
    private let lock = NSLock()
    private var traces: [String: Trace] = [:]

    private func running(_ name: String) -> Trace? {
        lock.lock()
        defer { lock.unlock() }
        return traces[name]
    }

    /// True when Firebase has a real project; otherwise warns once for `what`.
    private static func ready(_ what: String) -> Bool {
        guard SoarFirebase.configured else { SoarFirebase.warnOnce("perf.\(what)"); return false }
        return true
    }

    public func register(_ m: SoarModuleBuilder) {
        m.sync("configured(): boolean") { _ in SoarFirebase.configured }
        m.sync("startTrace(name: string): void") { [self] args in
            let name = args[0] as! String
            guard name.range(of: "^[A-Za-z0-9][A-Za-z0-9_]{0,99}$", options: .regularExpression) != nil else {
                throw Failure.invalid("invalid trace name `\(name)`")
            }
            guard Self.ready("startTrace"), running(name) == nil, let trace = Performance.startTrace(name: name) else {
                return nil
            }
            lock.lock()
            traces[name] = trace
            lock.unlock()
            return nil
        }
        m.sync("stopTrace(name: string): void") { [self] args in
            lock.lock()
            let trace = traces.removeValue(forKey: args[0] as! String)
            lock.unlock()
            trace?.stop()
            return nil
        }
        m.sync("incrementMetric(trace: string, metric: string, by: number): void") { [self] args in
            running(args[0] as! String)?.incrementMetric(args[1] as! String, by: (args[2] as! NSNumber).int64Value)
            return nil
        }
        m.sync("putAttribute(trace: string, key: string, value: string): void") { [self] args in
            running(args[0] as! String)?.setValue(args[2] as! String, forAttribute: args[1] as! String)
            return nil
        }
        m.sync("setEnabled(enabled: boolean): void") { args in
            if Self.ready("setEnabled") { Performance.sharedInstance().isDataCollectionEnabled = args[0] as! Bool }
            return nil
        }
    }
}
