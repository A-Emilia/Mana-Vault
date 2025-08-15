---
//marp: true
---

# Priority Tiers
 - Add Card (1), Remove Card (4), Create Collection (6), Create Deck (8), Delete Deck & Collection (14)
 - Search Card (2), Select Card (3), Update Collection (7)
 - Edit Card (5), Format (10)
 - Share Deck (11), See Public Decks (12)
 - Find & Replace (9), Sort Public Decks (13) Add Editors (15)

---
## To-Do List 
- [ ] Client
    - [ ] Request Builder
    - [ ] Response Handler
    - [ ] GUI
        - **Mockable in Figma**
        - [ ] Log-In
        - [ ] Main Page
        - [ ] Collection Page
        - [ ] Deck Page
        - [ ] Search Page
- [ ] Server
    - [ ] Response Struct
        - [ ] Response Builder
    - [ ] Request Handler
        - [ ] Request Struct
        - [ ] A way to differentiate between different request methods.
    - [ ] Server Querying of Database
    - [ ] Threadding
        - [ ] When a client is started or when a request is sent?
    - [ ] Structs
        - [ ] User
        - [ ] Main Collection
        - [ ] Collection
        - [ ] Deck
        - [ ] Rule(?)
        - [ ] Format(?)
        - [ ] (Card)
    - [ ] Data Storage
- [ ] General
    - [ ] Methods as described in the requirements.
    - [ ] Methods for non-requirements. 

---
## Roadmap
1. [] Successfully send a list of objects from the server to the client after receiving a request for them.
    - Implement a GET endpoint that is connected directly to the database. Fetches by ID.
2. [] Create the new database and migrate relevant data.
3. [] Successfully get all cards from the Alpha set that are legal with Inalla, Archmage Ritualist as the commander (including banned cards).
4. [] Successfully save a deck and then load it again from the database.