# Dokumentasjon

## Innholdsbetegnelse
- [Om Oppgaven](#om-oppgaven)
    - [Hva](#hva)
        - [Hva betegnelse](#hva-betegnelse)
        - [Netforum](#netforum)
        - [Database](#database)
        - [Hva man skal kunne gjøre](#hva-man-skal-kunne-gjøre)
            - [Bruker skal kunne](#brukere-skal-kunne)
            - [Admin skal kunne](#administratorer-skal-kunne)

    - [Hvordan](#hvordan)
    - [Hvorfor](#hvorfor)

## For lærlinger
Denne delen går over hva som er viktig for deg om du er lærling.

## Om Oppgaven
Denne seksjonen er om selve oppgaven [hva](#hva) oppgaven er, [hvordan](#hvordan) jeg utførte oppgaven og [hvorfor](#hvorfor) begrunnelser for hva jeg har gjort og valgene jeg har tatt.

### Hva 
Denne seksjonen handler om funksjonalitet og hva jeg skal bruke for å oppnå det ønsket resultatet.

#### Hva betegnelse
- [Netforum](#netforum)
- [Database](#database)
- [Funksjonalitet](#hva-man-skal-kunne-gjøre)
    - [Bruker](#brukere-skal-kunne)
    - [Administrator](#administratorer-skal-kunne)

#### Netforum
Jeg skal lage et nettforum med det jeg anser som standard nettforum funksjonalitet, eks. lage brukere, poste, svare andre, etc. Brukerer skal autentiseres med JWT (Json Web Tokens). Passord skal hashes og saltes før de lagrer i databasen. Nettsiden skal ikke ha et konsept eller noe som gjør at den skiller seg ut.

#### Database
Det er en selv hostet mongodb database. Databasen skal være hostet på en hyper-v som kjører Debian Bookworm. Databaseoppsette skal være lagret i en .js fil.

#### Hva man skal kunne gjøre
Siden skal ha noen funksjoner de viktigste er listed nedenfor

##### Brukere skal kunne:
- lage bruker 
- personalisere brukeren
- poste threads på boards
- poste på threads
- svare på andre brukere sine poster 
- komme med reaksjoner

##### Administratorer skal kunne:
- slette brukere, threads og posts
- ip banning
- se alle brukere 
- pinne poster
- lage boards

Teknologier/Stack/Alternativer

|Bruksomeråde|Navn på teknologi|Alternativer|
|----|----|----|
|Backendspråk|Rust|C#|
|Webserver rammeverk|Actxi-web|Tokyo|
|Frontendspråk|React m/TypeScript|Vue, Plain|
|Bundle server|Vite|Nextjs|
|Database|MongoDB|PostGres|

### Hvordan
Jeg skal gjøre dette med min forståelse av Actix-web og Rust. Bruke det jeg kan om MongoDB, viritualisering og Debian for å sette opp serveren. Siden jeg ikke trenger en statisk filserver så skal jeg ikke bruke noe som Apache2 eller NGINX. De kan fortsatt ha nytte til prosjektet mitt ved å bli brukt som reverse proxy.

### Hvorfor
Grunnen til at jeg vil lage et nettforum er fordi alle netforumene jeg har brukt i det siste er dårlig alle bruker samme layout jeg ikke liker. Jeg liker også teknologiene og ønsker å bruke dem mer og få en bedre forståelse for hvordan det fungerer og måtene man kan bruke dem på.

Valg av programmeringsspråk handler om at jeg er kjent med Rust og har brukt en del Actix-web tidligere.

Grunnen til at jeg bruker vite og react på frontenden min er at de fleste bedrifter og store steder bruker React så det er bra og kunne. Det har mye bra funksjonalitet vite er enkelt å sette opp så det gjør det lettere å komme i gang.

## Kode dokumentasjon
I denne seksjonen skal jeg forklare funksjonalitet i koden min og hvordan koden henger sammen.

### Frontend
Frontenden min er skrevet med TypeScript og React. TypeScript er en typecasted version av JavaScript. Altså du må spesifisere typene på vaiabler, om de er tall strenger eller array også videre.

Målet med frontenden er å vise fram ting som jeg legger ut på endepunketene mine. Helt ideelt skal frontendene være utbyttbare og du skal kunne velge mellom hvilke frontend en øsnker å bruke. Denne dokumenatasjonen går over React frontenden.

#### Kode

### Backend

#### Kode