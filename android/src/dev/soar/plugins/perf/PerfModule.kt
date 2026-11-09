package dev.soar.plugins.perf

import android.content.Context
import com.google.firebase.perf.FirebasePerformance
import com.google.firebase.perf.metrics.Trace
import dev.soar.SoarModule
import dev.soar.SoarModuleBuilder
import dev.soar.plugins.firebase.SoarFirebase

/** `native:perf` over Firebase Performance Monitoring (soar-plugin-firebase-perf). */
class PerfModule(context: Context) : SoarModule {
    override val name = "perf"
    private val app = context.applicationContext

    /** Running traces by name. Module functions may run on the bridge thread, so access is synchronized. */
    private val traces = HashMap<String, Trace>()

    /** The SDK, or null (after a one-time warning for [what]) when Firebase has no real project. */
    private fun sdk(what: String): FirebasePerformance? {
        if (!SoarFirebase.configured(app)) { SoarFirebase.warnOnce("perf.$what"); return null }
        return FirebasePerformance.getInstance()
    }

    private fun running(name: String): Trace? = synchronized(traces) { traces[name] }

    override fun register(m: SoarModuleBuilder) {
        m.sync("configured(): boolean") { _ -> SoarFirebase.configured(app) }
        m.sync("startTrace(name: string): void") { args ->
            val name = args.getString(0)
            require(Regex("[A-Za-z0-9][A-Za-z0-9_]{0,99}").matches(name)) { "invalid trace name `$name`" }
            val perf = sdk("startTrace")
            if (perf != null && running(name) == null) {
                val trace = perf.newTrace(name)
                trace.start()
                synchronized(traces) { traces[name] = trace }
            }
            null
        }
        m.sync("stopTrace(name: string): void") { args ->
            synchronized(traces) { traces.remove(args.getString(0)) }?.stop()
            null
        }
        m.sync("incrementMetric(trace: string, metric: string, by: number): void") { args ->
            running(args.getString(0))?.incrementMetric(args.getString(1), args.getLong(2))
            null
        }
        m.sync("putAttribute(trace: string, key: string, value: string): void") { args ->
            running(args.getString(0))?.putAttribute(args.getString(1), args.getString(2))
            null
        }
        m.sync("setEnabled(enabled: boolean): void") { args ->
            sdk("setEnabled")?.isPerformanceCollectionEnabled = args.getBoolean(0)
            null
        }
    }
}
