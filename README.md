## Meta Tic-Tac-Toe

### Overview

A server-authoritative, scalable multiplayer game server written in Rust, supporting:

* Variable board sizes
* Real-time multiplayer
* Persistent player ratings
* Cloud-native deployment

### Architecture

```
           ┌────────────┐
           │   Browser  │
           │  (Yew UI)  │
           └─────┬──────┘
                 │ WebSocket / WebRTC
        ┌────────▼─────────┐
        │   Axum Server    │
        │ (Game Authority) │
        └────────┬─────────┘
                 │
        ┌────────▼─────────┐
        │     Redis        │
        │ Ratings / Games  │
        └──────────────────┘
```

### Game Flow

* Player creates or joins a room
* Server assigns symbols
* Server validates all moves
* Game result updates Elo
* Results persisted in Redis

### 🧠 Win-Check Algorithm (Variable Grid)

**Rules**

* **Win condition** = N in a row
* **N = grid_size** (3, 5, or 7)
* **Check:**
    * **Rows**
    * **Columns**
    * **Diagonals**

* Win condition = N in a row
* `N = grid_size` (3, 5, or 7)
* Check:
  * Rows
  * Columns
  * Diagonals

### 🧠 Redis Persistence

* Player profiles
* Elo ratings
* Active games (TTL)
* Match history

### 🌐 WebRTC (Optional Optimization)

* Used after matchmaking
* Server stays authoritative
* WebSocket fallback always active
* Excellent discussion point in interviews
