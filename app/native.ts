const { nativeStartTokioRuntime, nativeShutdownTokioRuntime } = require('./native.node');

export function startTokioRuntime() {
  nativeStartTokioRuntime();
}

export function shutdownTokioRuntime() {
  nativeShutdownTokioRuntime();
}
