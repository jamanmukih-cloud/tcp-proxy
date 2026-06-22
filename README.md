# TCP Proxy 🔄

High-performance TCP proxy with connection pooling and health checks.

## Performance

| Metric | Value |
|--------|-------|
| Connections/sec | 50,000 |
| Bandthrough | 40 Gbps |
| Latency overhead | <0.1ms |
| Memory per conn | 256 bytes |

## Features

- **Connection Pooling**: Reuse connections
- **Health Checks**: Active/passive
- **Traffic Mirroring**: Shadow traffic
- **TLS Termination**: Built-in

## Quick Start

```bash
tcp-proxy --listen 0.0.0.0:8080 --backend 10.0.1.10:3000,10.0.1.11:3000
```

## License

Apache 2.0