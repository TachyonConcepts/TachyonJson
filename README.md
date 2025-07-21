# Tachyon JSON

**Tachyon JSON** is a hyper-optimized JSON serializer and buffer written in Rust.  
It's not just fast — it *doesn't apologize for hurting nanoseconds*.

---

## 🚀 Mission

To build a serializer that:

- Allocates nothing
- Copies nothing extra
- Escapes only if you say so
- Doesn't slow down
- Doesn't say sorry

---

## 📈 Performance

Honest microbenchmark results (`1_000_000` iterations):

{"message":"Hello, world!"}

Serde (zero-copy) at: 23 ns.

Tachyon JSON at: 8 ns.

Yes, you read that right.

---

## 🛠 Usage

```rust
let mut out = TachyonBuffer::<100>::default();

let msg = tachyon_object_noescape! {
    "message" => "Hello, world!",
};

unsafe { msg.encode(&mut out, true); }

println!("{}", String::from_utf8_lossy(out.as_slice()));
