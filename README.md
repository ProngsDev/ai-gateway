# 🚀 Rust AI Gateway

A lightning-fast, production-ready API gateway that unifies OpenAI and Google Gemini models into a single endpoint. Built with Rust for maximum performance and reliability.

## ✨ Features

- **🔄 Multi-Provider Support** - Seamlessly integrate OpenAI and Gemini
- **⚡ Automatic Failover** - Falls back to secondary provider if primary fails
- **💾 Smart Caching** - In-memory cache for improved performance and reduced costs
- **🎯 Manual Provider Selection** - Override automatic routing when needed
- **📊 Request Logging** - Structured logging with tracing for observability
- **🔒 Type-Safe** - Leverages Rust's type system for reliability
- **⚙️ Async/Await** - Non-blocking I/O for high concurrency

## 🏗️ Architecture

```
┌─────────────┐
│   Client    │
└──────┬──────┘
       │
       ▼
┌─────────────────────────────────┐
│    Axum API Gateway             │
│  /generate endpoint             │
└──────┬──────────────────────────┘
       │
       ▼
┌─────────────────────────────────┐
│   Router (model selection)      │
│   - Automatic failover          │
│   - Manual override             │
└──────┬──────────────────────────┘
       │
       ├──────────────┬─────────────┐
       ▼              ▼             ▼
┌──────────┐   ┌──────────┐   ┌─────────┐
│ OpenAI   │   │  Gemini  │   │  Cache  │
│ Client   │   │  Client  │   │  Layer  │
└──────────┘   └──────────┘   └─────────┘
```

## 🚀 Quick Start

### Prerequisites

- Rust 1.70+ ([Install Rust](https://rustup.rs/))
- OpenAI API Key ([Get one here](https://platform.openai.com/api-keys))
- Google Gemini API Key ([Get one here](https://makersuite.google.com/app/apikey))

### Installation

1. **Clone the repository**
   ```bash
   git clone https://github.com/ProngsDev/ai-gateway.git
   cd ai-gateway
   ```

2. **Set up environment variables**
   ```bash
   cp .env.example .env
   ```

   Edit `.env` with your API keys:
   ```env
   PORT=8080
   OPENAI_API_KEY=sk-your-openai-key-here
   GEMINI_API_KEY=your-gemini-key-here
   ```

3. **Build and run**
   ```bash
   cargo run --release
   ```

The server will start on `http://localhost:8080`

## 📖 API Reference

### Health Check

```bash
GET /health
```

**Response:**
```
AI Gateway is healthy
```

---

### Generate Text

```bash
POST /generate
Content-Type: application/json
```

**Request Body:**
```json
{
  "prompt": "Explain Rust ownership in one sentence",
  "provider": "OpenAI"  // Optional: "OpenAI" or "Gemini"
}
```

**Response:**
```json
{
  "provider": "OpenAI",
  "output": "Rust's ownership system ensures memory safety by enforcing that each value has a single owner, and when the owner goes out of scope, the value is automatically deallocated.",
  "cached": false
}
```

**Fields:**
- `provider` (optional): Specify "OpenAI" or "Gemini". If omitted, automatic failover is used.
- `prompt` (required): The text prompt to send to the AI model.

**Response Fields:**
- `provider`: Which provider generated the response
- `output`: The generated text
- `cached`: Whether the response was served from cache

## 💡 Usage Examples

### Automatic Failover (Recommended)

```bash
curl -X POST http://localhost:8080/generate \
  -H "Content-Type: application/json" \
  -d '{
    "prompt": "What is Rust?"
  }'
```

The gateway will:
1. Try OpenAI first
2. If OpenAI fails, automatically fall back to Gemini
3. Cache the successful response

---

### Manual Provider Selection

**Use OpenAI specifically:**
```bash
curl -X POST http://localhost:8080/generate \
  -H "Content-Type: application/json" \
  -d '{
    "prompt": "Explain async/await",
    "provider": "OpenAI"
  }'
```

**Use Gemini specifically:**
```bash
curl -X POST http://localhost:8080/generate \
  -H "Content-Type: application/json" \
  -d '{
    "prompt": "Explain async/await",
    "provider": "Gemini"
  }'
```

---

### Cache Behavior

Send the same prompt twice:

```bash
# First request - hits OpenAI
curl -X POST http://localhost:8080/generate \
  -H "Content-Type: application/json" \
  -d '{"prompt": "Hello"}' | jq

# Response: {"provider": "OpenAI", "output": "...", "cached": false}

# Second request - served from cache
curl -X POST http://localhost:8080/generate \
  -H "Content-Type: application/json" \
  -d '{"prompt": "Hello"}' | jq

# Response: {"provider": "OpenAI", "output": "...", "cached": true}
```

## 🛠️ Development

### Run Tests

```bash
cargo test
```

### Run with Debug Logging

```bash
RUST_LOG=debug cargo run
```

### Build for Production

```bash
cargo build --release
```

The optimized binary will be in `target/release/ai-gateway`

## 🐳 Docker Deployment

### Build Docker Image

```bash
docker build -t ai-gateway .
```

### Run Container

```bash
docker run -p 8080:8080 \
  -e OPENAI_API_KEY=your-key \
  -e GEMINI_API_KEY=your-key \
  ai-gateway
```

### Docker Compose

```bash
docker-compose up -d
```

See [Docker Setup](#docker) section for details.

## 📁 Project Structure

```
ai-gateway/
├── src/
│   ├── main.rs              # Server setup & initialization
│   ├── routes.rs            # API endpoint handlers
│   ├── router.rs            # Provider routing & failover logic
│   ├── cache.rs             # In-memory cache implementation
│   ├── error.rs             # Custom error types
│   └── providers/
│       ├── mod.rs           # AIProvider trait definition
│       ├── openai.rs        # OpenAI client implementation
│       └── gemini.rs        # Gemini client implementation
├── Cargo.toml               # Dependencies & metadata
├── .env                     # Environment variables (gitignored)
├── Dockerfile               # Docker build configuration
├── docker-compose.yml       # Docker Compose setup
└── README.md                # This file
```

## 🔧 Configuration

### Environment Variables

| Variable | Description | Required | Default |
|----------|-------------|----------|---------|
| `PORT` | Server port | No | `8080` |
| `OPENAI_API_KEY` | OpenAI API key | Yes | - |
| `GEMINI_API_KEY` | Google Gemini API key | Yes | - |

### Provider Priority

The default failover order is:
1. **OpenAI** (primary)
2. **Gemini** (fallback)

To change this, modify the provider order in `src/main.rs`:

```rust
// Current order (OpenAI first)
ai_router.add_provider(openai_client);
ai_router.add_provider(gemini_client);

// To make Gemini primary, swap the order:
ai_router.add_provider(gemini_client);
ai_router.add_provider(openai_client);
```

## 🎯 Use Cases

- **SaaS Applications** - Add AI features with built-in resilience
- **Cost Optimization** - Route to cheaper providers, cache expensive calls
- **High Availability** - Automatic failover ensures uptime
- **Multi-Model Apps** - Leverage strengths of different models
- **Development/Testing** - Single API for multiple providers

## 🔒 Security Considerations

- Store API keys in environment variables, never in code
- Use HTTPS in production
- Consider adding rate limiting for public deployments
- Implement authentication/authorization as needed

## 📊 Performance

- **Async I/O** - Non-blocking requests for high concurrency
- **In-Memory Cache** - Sub-millisecond cache hits
- **Compiled** - Native performance with Rust
- **Lightweight** - Minimal resource footprint

## 🤝 Contributing

Contributions are welcome! Areas for improvement:

- Add more providers (Claude, Llama, Cohere)
- Implement Redis caching for distributed systems
- Add rate limiting
- Prometheus metrics
- Streaming responses (SSE)
- Configuration via YAML/TOML

## 📝 License

MIT License - See LICENSE file for details

## 🙏 Acknowledgments

Built with:
- [Axum](https://github.com/tokio-rs/axum) - Web framework
- [Tokio](https://tokio.rs/) - Async runtime
- [Reqwest](https://github.com/seanmonstar/reqwest) - HTTP client
- [Serde](https://serde.rs/) - JSON serialization
- [Tracing](https://github.com/tokio-rs/tracing) - Logging

---

**Made with ❤️ and Rust**
