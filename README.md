# Istio与WebAssembly

Istio引入了WebAssembly扩展的概念，它允许开发者通过将自定义的WebAssembly模块插入Istio的Envoy代理来扩展Istio的功能。这样，开发者可以在Istio代理中运行自己的逻辑，并且可以通过WebAssembly模块来修改和定制网络请求、响应以及流量控制。这为Istio带来了更高的灵活性和可扩展性，允许开发者在不改变Istio核心代码的情况下添加自定义功能。

WebAssembly在Istio中的应用场景包括但不限于：

- 自定义流量管理：开发者可以使用WebAssembly模块来实现自定义的流量控制策略，如AB测试、灰度发布等。
- 安全策略：通过WebAssembly模块，可以实现自定义的安全策略，例如访问控制、防火墙规则等。
- 日志和监控：开发者可以使用WebAssembly模块来收集特定流量的指标或日志。

## ABI 规范定义 (Application Binary Interface)

应用程序二进制接口（ABI）的规范，定义在L4/L7代理（和/或其他主机环境）与作为WebAssembly模块交付的扩展之间使用的约定。

这些基于事件驱动的流式API和便捷的实用函数最初是为WebAssembly在Envoy项目中开发的，但它们与代理无关，使用者可以在不同的代理之间使用相同的Proxy-Wasm扩展。

### SDKs

- [C++ SDK](https://github.com/proxy-wasm/proxy-wasm-cpp-sdk)
- [Rust SDK](https://github.com/proxy-wasm/proxy-wasm-rust-sdk)
- [AssemblyScript SDK](https://github.com/solo-io/proxy-runtime)
- [TinyGo SDK](https://github.com/tetratelabs/proxy-wasm-go-sdk)

## 环境准备

```shell
wget https://github.com/labring/sealos/releases/download/v4.3.0/sealos_4.3.0_linux_amd64.tar.gz
tar -zxvf sealos_4.3.0_linux_amd64.tar.gz sealos
chmod a+x sealos 
mv sealos /usr/bin/
```

```shell
sealos run labring/kubernetes-docker:v1.23.0 labring/helm:v3.12.0 labring/calico:v3.24.1
sealos run labring/istio:1.16.2-min
```
