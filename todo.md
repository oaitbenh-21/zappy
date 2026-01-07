# Zappy Game Logic - Complete Implementation Guide

## 📋 Overview

This document serves as your game engine specification checklist for implementing Zappy, a multiplayer tile-based strategy game with resource management, evolution mechanics, and team-based victory conditions.

---

## 1️⃣ World Initialization Logic

### Task 1.1 — World Creation

**What to do:**
- Create a 2D world with width `x` and height `y`
- Represent it as a grid of tiles

**Rules:**
- No obstacles
- World is toroidal

**Explanation:**  
The map wraps around on all edges; moving out on one side re-enters on the opposite side.

### Task 1.2 — Toroidal Coordinate Handling

**What to do:**  
Normalize any `(x, y)` position

**Rule:**
```
x = (x + width) % width
y = (y + height) % height
```

**Explanation:**  
This guarantees infinite looping geography.

---

## 2️⃣ Resource Generation Logic

### Task 2.1 — Resource Types Definition

**What to do:**  
Define 6 stone types + food

**Explanation:**  
Objects are class-based, not instance-based.

### Task 2.2 — Resource Placement Rules

**What to do:**  
Randomly place resources at world creation

**Constraints:**
- Max 1 food per tile
- Max 1 stone per type per tile
- Max 3 stones per tile total
- No tile can hold all stones

**Explanation:**  
Ensures fairness and avoids trivial clustering.

### Task 2.3 — Resource Regeneration (Optional but Recommended)

**What to do:**  
Periodically regenerate resources

**Explanation:**  
Prevents dead maps; keeps game playable long-term.

---

## 3️⃣ Player Creation & State Logic

### Task 3.1 — Player Spawn

**What to do:**  
Spawn player at random position and direction

**Initial State:**
- Level 1
- 10 food
- Empty stone inventory

### Task 3.2 — Player Orientation

**What to do:**  
Handle 4 directions (N/E/S/W)

**Rules:**  
`left` / `right` rotate 90°

---

## 4️⃣ Time & Scheduling Logic ⚠️ CRITICAL

### Task 4.1 — Time Unit System

**What to do:**  
Define `1 time unit = 1 / t seconds`

**Explanation:**  
All actions are scaled by `t`.

### Task 4.2 — Action Queue Per Player

**What to do:**  
Each player has a FIFO queue of actions

**Rules:**
- Max 10 pending actions
- Actions execute after their delay
- Only block the player, not the server

### Task 4.3 — Action Execution

**What to do:**  
When time expires:
1. Apply game logic
2. Send result (`ok`, `ko`, data)

---

## 5️⃣ Food & Survival Logic

### Task 5.1 — Food Consumption

**What to do:**  
Decrease food over time

**Rule:**  
`1 food = 126 time units`

### Task 5.2 — Starvation Death

**What to do:**  
Kill player when food reaches zero

**Effects:**
- Remove from world
- Cancel ritual participation
- Notify client

---

## 6️⃣ Movement Logic

### Task 6.1 — Advance

**What to do:**  
Move one tile forward

**Rules:**
- Apply toroidal wrap
- Takes `7 / t`

### Task 6.2 — Collision Handling

**What to do:**  
Multiple players can occupy same tile

**Explanation:**  
There is no blocking.

---

## 7️⃣ Vision Logic

### Task 7.1 — Vision Range Computation

**What to do:**  
Based on player level

**Pattern:**  
`Level N → N rows ahead`

### Task 7.2 — Vision Serialization

**What to do:**  
Build response string `{square0, square1, ...}`

**Rules:**
- Do not include self
- Separate multiple items with spaces

---

## 8️⃣ Inventory Logic

### Task 8.1 — Inventory Structure

**What to do:**  
Track food + stones

### Task 8.2 — Inventory Command

**What to do:**  
Return all objects and quantities

---

## 9️⃣ Pick & Drop Logic

### Task 9.1 — Pick Object

**What to do:**  
Transfer object from tile to inventory

**Failure cases:**
- Object doesn't exist
- Inventory limit violated (if any)

### Task 9.2 — Drop Object

**What to do:**  
Transfer object from inventory to tile

**Failure cases:**
- No such object

---

## 🔟 Kick Logic

### Task 10.1 — Kick Execution

**What to do:**  
Push all players on tile (except self)

**Rules:**
- Direction = kicker's orientation
- Cannot kick during enchantment

### Task 10.2 — Kick Notification

**What to do:**  
Send `moving <K>` to kicked players

---

## 1️⃣1️⃣ Broadcast & Sound Logic ⚠️ IMPORTANT

### Task 11.1 — Broadcast Reception

**What to do:**  
Send message to all players

### Task 11.2 — Sound Direction Computation

**What to do:**  
Compute shortest toroidal path

**Rules:**
- Translate into direction number 0–8
- Relative to receiver orientation

---

## 1️⃣2️⃣ Enchantment (Evolution) Logic ⚠️ VERY IMPORTANT

### Task 12.1 — Level Requirements Table

**What to do:**  
Encode exact stone + player requirements

### Task 12.2 — Enchantment Start

**What to do:**  
Check:
- Same tile
- Same level
- Required stones
- At least 2 players

### Task 12.3 — Enchantment Duration

**What to do:**  
Block participants for `300 / t`

### Task 12.4 — Enchantment Failure Handling

**What to do:**
- If initiator alone → cancel
- If someone dies → continue if valid

### Task 12.5 — Level Up

**What to do:**
- Consume stones
- Increase level of all participants

---

## 1️⃣3️⃣ Team & Fork Logic

### Task 13.1 — Teams Creation

**What to do:**  
Create teams at server start

### Task 13.2 — Fork Action

**What to do:**
- Schedule egg creation
- Spawn new player after `600 / t`

### Task 13.3 — connect_nbr

**What to do:**  
Return remaining slots for team

---

## 1️⃣4️⃣ Win Condition Logic

### Task 14.1 — Victory Detection

**What to do:**  
Detect 6 players of same team at level 8

### Task 14.2 — Game End

**What to do:**
- Stop game loop
- Notify all clients

---

## 1️⃣5️⃣ Error Handling Logic

### Task 15.1 — Invalid Command

**What to do:**  
Return `ko`

### Task 15.2 — Rule Violations

**What to do:**
- Never crash
- Never block
- Never corrupt state

---

## 🎯 Final Mental Checklist (Auditor Level)

If you can answer **YES** to all of these, you're done:

- ✔ Can I simulate the entire game without sockets?
- ✔ Is every rule enforced server-side?
- ✔ Is time deterministic?
- ✔ Can players act in parallel?
- ✔ Does starvation, evolution, and kicking interact correctly?

---

## 🔥 Implementation Order (Strong Advice from Experience)

Implement in this order:

1. **World + player structs**
2. **Time + scheduler**
3. **Movement + food**
4. **Inventory + vision**
5. **Broadcast + kick**
6. **Enchantment**
7. **Win condition**

---

## 📝 Notes

This specification ensures:
- Deterministic gameplay
- Non-blocking concurrent actions
- Robust state management
- Fair resource distribution
- Clear victory conditions

Follow this checklist systematically to build a complete, bug-free Zappy server implementation.