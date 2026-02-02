# sonar

A lightweight Docker container metrics exporter for Prometheus, written in Rust.

## Features

- Exports CPU and memory usage metrics per container
- Prometheus-compatible output format
- Low resource footprint

## Requirements

- Docker API v1.41+

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

- `GET /metrics`
  - **Description:** Returns Prometheus-formatted metrics for all running containers.
  - **Response Example:**
    ```text
    # HELP container_cpu_usage_percent CPU usage percentage
    # TYPE container_cpu_usage_percent gauge
    container_cpu_usage_percent{container="sonar"} 0.12
    container_memory_usage_bytes{container="sonar"} 123456
    ...
    ```

- `GET /json`
  - **Description:** Returns raw container stats in JSON format.
  - **Response Schema:**
    ```json
    [
    	{
    		"id": "<container_id>",
    		"name": "<container_name>",
    		"cpu_usage_percent": 0.12,
    		"memory_usage_percent": 0.5,
    		"memory_usage_bytes": 123456
    	},
    	...
    ]
    ```

## Docker Image & Resource Usage

- **Image Size:** ~4.5 MB (compressed)
- **Runtime Memory Usage:** 700–900 KB RAM

This makes the container lightweight and suitable for resource-constrained environments.

## License

MIT
