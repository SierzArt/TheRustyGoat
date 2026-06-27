# TheRustyGoat
The first open-source thin-client cloud phone OS • Device = beautiful screen + input • Everything else runs in the cloud • Built in pure Rust
# ☁️ Aether Terminal • The First True Cloud Phone OS (Rust)

> The phone is just a beautiful screen.  
> The brain lives in the cloud.

**Most "cloud phone" projects** run full Android in a server and let you remote-control it with a normal phone app.

**This is different.**

We are building a **dedicated thin-client device + minimal Rust OS** where the hardware is intentionally dumb and the entire experience runs in your own cloud.

### Vision
- Unlimited compute, RAM, AI, battery life
- Perfect consistency across all your devices
- Extremely high security (tiny attack surface)
- Open, private, and community-driven

### Current Status (Day 1 — June 2026)
- ✅ Minimal Rust kernel (`no_std` + aarch64)
- ✅ Framebuffer + display init (beautiful boot screen)
- ✅ Placeholders for networking, streaming, input forwarding

### Tech Stack
- Pure Rust (`no_std` + embedded-graphics)
- Target: ARM64 (phones, dev boards, custom hardware)
- Planned: QUIC/WebTransport + AV1 streaming + hardware attestation + predictive input

