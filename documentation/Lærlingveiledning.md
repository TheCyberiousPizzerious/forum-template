# Veiledning for lærling

Hei lærling, dette skal være en veiledning for at du skal kunne ta over!

Jeg antar at du vet litt om programmeringsspråkene og teknologien som driver Something Forum.

## Stack
Rust - språket som driver APIen og backenden generelt
    Actix Web - Rammeverk som brukes for å host apien

React - språket som driver frontenden ansvarlig for det visuelle 
    Tailwind css - Css rammeverk som fjerner bruken for index.css filer

## Frontend
Som sagt drives frontenden av React. De fleste filene i frontenden er boilerplate som bare er nødvendig for at React skal fungere. Andre filer derimot trenger du å redigere en gang i blant det er som følger:
- vite.config.ts - ip og port 
- tailwind.config.js - egendefinerte farger og fonter

### Kode
Strukturen er som følger: 
- src/
    - assets/
    - components/
    - pages/
App.tsx
index.css
main.tsx

Store deler av dette forklarer seg selv. assets er for ting som bilder og medie brukt på siden. Components er for komponenter som skal brukes på siden som igjen er definert under pages. App.tsx håndterer Routene de andre filene i src er boilerplate

## Backend
Rust er språket igjen som driver backenden, siden Rust ikke egentlig er veldig einet for apier. Det er en innebygget webserver crate, men den er ikke for APIer så vi bruker Actix Web for å lage endepunkter.

### Kode
Strukturen er som følger:
- src/
    - controllers/
    - models/
    - mongo_repo/
    - utils/
main.rs

Så lenge du har minimal forståelse av Rust kode er ikke det noe problem. controllers inne holder filer som skal håndtere forespørrsler. models har filer for modeller som er nødvendig siden Rust er så type heavy. mongo_repo innkluderer filer og funksjoner som er relatert til mongodb som klient linken. Main.rs som i alle andre Rust programmer er koden som skal kjøre når vi starter programmet. 

Rust programmet er brutt ned i flere mindre modules, men vi trenger fortsatt at MongoDB klienten skal kunne nås blant alle filene. Får å oppnå dette bruker vi en "Arc" Dette gjør at så lenge vi importerer denne Arcen til den ønskede filen har vi tilgang til variablen som blir "Arcet" Den er wrappet i "Data" typen er så vi kan bruke den som en del av våre endepunkt funksjoner.