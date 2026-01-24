# sonar

A lightweight Docker container metrics exporter for Prometheus, written in Rust.

## Features

- Exports CPU and memory usage metrics per container
- Prometheus-compatible output format
- Low resource footprint

## Requirements

- Docker API v1.47+

## Usage

```bash
sonar --port 3000 --socket-path /var/run/docker.sock
```

### Docker

```bash
docker run -v /var/run/docker.sock:/var/run/docker.sock -p 3000:3000 ababarytskyi/sonar:latest
```

Or build from source:

```bash
docker build -t sonar .
docker run -v /var/run/docker.sock:/var/run/docker.sock -p 3000:3000 sonar
```

## Metrics

| Metric                           | Description             |
| -------------------------------- | ----------------------- |
| `container_cpu_usage_percent`    | CPU usage percentage    |
| `container_memory_usage_percent` | Memory usage percentage |
| `container_memory_usage_bytes`   | Memory usage in bytes   |

## Endpoints

- `GET /metrics` - Prometheus metrics
- `GET /json` - Raw JSON stats

## License

MIT
