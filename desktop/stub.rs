//! Desktop stand-in for `native:perf` (soar-plugin-firebase-perf): there is no Performance Monitoring on desktop,
//! so the calls do nothing and `configured()` is false.
//!
//! The file depends only on the host API, so it builds wherever the plugin folder sits in a Soar checkout.

use soar_app::host::{Host, ModuleBuilder, NativeModule, RegisterError, Value};

struct Stub;

impl NativeModule for Stub {
    fn name(&self) -> &str {
        "perf"
    }

    fn register(&self, m: &mut ModuleBuilder<'_>) -> Result<(), RegisterError> {
        m.sync("configured(): boolean", |_| Ok(Value::Bool(false)))?
            .sync("startTrace(name: string): void", |_| Ok(Value::Null))?
            .sync("stopTrace(name: string): void", |_| Ok(Value::Null))?
            .sync("incrementMetric(trace: string, metric: string, by: number): void", |_| Ok(Value::Null))?
            .sync("putAttribute(trace: string, key: string, value: string): void", |_| Ok(Value::Null))?
            .sync("setEnabled(enabled: boolean): void", |_| Ok(Value::Null))?;
        Ok(())
    }
}

/// Registers the stand-in.
pub fn register(host: &mut Host) {
    host.add(&Stub).expect("native:perf signatures match perf.d.ts");
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The stand-in must link with what the declarations promise, or apps importing `native:perf` fail to load.
    #[test]
    fn an_app_using_perf_links() {
        let bundle = soar_compiler::compile_with(
            r#"
            import * as perf from "native:perf";
            export default function App() {
              function tap() {
                perf.startTrace("login");
                perf.putAttribute("login", "method", "password");
                perf.incrementMetric("login", "attempts", 1);
                perf.stopTrace("login");
                perf.setEnabled(true);
              }
              return <Pressable onPress={tap}><Text>ok</Text></Pressable>;
            }
        "#,
            include_str!("../perf.d.ts"),
        )
        .unwrap();
        let mut host = Host::new();
        register(&mut host);
        let mut ui = soar_runtime::Ui::new(1.0);
        ui.resize(200, 200, 1.0);
        let root = ui.scene.root();
        soar_app::App::with_host(&mut ui, root, bundle, std::sync::Arc::new(host)).unwrap();
        ui.frame_into(&mut Vec::new());
    }

    #[test]
    fn calls_do_nothing_and_report_unconfigured() {
        let mut host = Host::new();
        register(&mut host);
        assert_eq!(host.call_sync("perf", "configured", &[]), Some(Ok(Value::Bool(false))));
        assert_eq!(host.call_sync("perf", "startTrace", &[Value::Str("login".into())]), Some(Ok(Value::Null)));
    }
}
