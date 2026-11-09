# soar-plugin-firebase-perf

Custom performance traces for [Soar](https://kakashi250802.github.io/soar-install/) apps through Firebase
Performance Monitoring: `native:perf` ([perf.d.ts](perf.d.ts)). It builds on the `firebase-core` plugin that
ships with Soar, and on your project's Firebase config files.

## Install

From a Soar checkout:

```bash
soar plugin add github:kakashi250802/soar-plugin-firebase-perf@v0.1.0
```

The repository is cloned into `plugins/vendor/firebase-perf` (git-ignored) and wired into the iOS, Android
and desktop hosts; `firebase-core` is installed first when it is missing. Remove it with
`soar plugin remove firebase-perf`. Update a clone with `git -C plugins/vendor/firebase-perf pull`, then
`soar plugin sync`.

## Use

```tsx
import * as perf from "native:perf";

perf.startTrace("login");
perf.putAttribute("login", "method", "password");
perf.incrementMetric("login", "attempts", 1);
perf.stopTrace("login");
```

Traces are named, so a screen can start one in an effect and stop it in a handler. An invalid trace name
throws (a rejected call in the handler). Without a Firebase project every call does nothing and logs
"Firebase not configured" once; on desktop the calls do nothing.

## What it adds

| Host | Adds |
| --- | --- |
| iOS | the `FirebasePerformance` product of `firebase-ios-sdk` 11.6.0 (Swift Package Manager) |
| Android | `com.google.firebase:firebase-perf` from the Firebase BoM of `firebase-core` |
| Desktop | a stand-in module that does nothing |

Automatic network and screen traces need the Firebase Performance Gradle plugin, which this plugin does not
apply yet; custom traces work without it.

## License

MIT
