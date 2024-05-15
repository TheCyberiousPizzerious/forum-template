# Dokumentasjon
Trygve A. Hareide, SomethingForum, 2024

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
    - [Kode dokumentasjon](#kode-dokumentasjon)
        - [Frontend](#frontend)


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

## Teknisk dokumentasjon
![Nettverkstegning](/documentation/Diagram.png)
Nettverksplanen min viser en klient som kontakter siden min går gjennom frontenden min logger inn og der sender til backenden min som igjen kontakter serveren min.

### Backup
Ønsket er rutinert backup av mongodb data, med brukerdata som høyeste prioritering og monitorering logger som laveste. 

#### Informasjon
|server|passord|ip/hostname|porter|funksjonalitet|
|---|---|---|---|---|
MongoDeBian|-,.32.dj90=s9G/|xxx|21707|mongodb database|
Host|xxx|xxx|7175|Rust server|

## Kode dokumentasjon
I denne seksjonen skal jeg forklare Hvordan man leser Rust kode og litt overfladisk hva koden gjør. Det samme med frontenden.

### Frontend
Frontenden min er skrevet med TypeScript og React. TypeScript er en typecasted version av JavaScript. Altså du må spesifisere typene på vaiabler, om de er tall strenger eller array også videre.

Målet med frontenden er å vise fram ting som jeg legger ut på endepunketene mine. Helt ideelt skal frontendene være utbyttbare og du skal kunne velge mellom hvilke frontend en øsnker å bruke. Denne dokumenatasjonen går over React frontenden.

React har mye innebygget funksjonalitet og sider fungerer ved at man setter sammen flere mindre komponenter som man finner i "components" mappen. Disse komponentene settes sammen på sidene som kan sees under "pages" mappen.

#### Kode
Det er noen viktige filer som er nødvendige for at react prosjektet fungerer, men noen ganger trenger du også å redigere disse. Noen viktige filer er:
- vite.config.ts - Filen inneholder serverspesifikasjoner som port og ip
- tailwind.config.js - Inneholder configurasjon for tailwind, ekstra farger og tilpassede skrifttyper
"Rutene" til prosjektet altså hvordan prosjektet vet hvilken fil den skal levere til hvilken url er bestemt i App.tsx. 
React har mye innebygget funksjonalitet og sider fungerer ved at man setter sammen flere mindre komponenter som man finner i "components" mappen. Disse komponentene settes sammen på sidene som kan sees under "pages" mappen. React bruker bare et html dokument og bygger alle sidene ut fra dette html dokumentet. I React laster man komponenter isteden for å laste hele siden.

### Backend
Backenden min er skrevet i Rust og jeg bruker rammeverket actix-web for å drive apien min. I Rust handler alt om typer. Det er veldig strenge regler rundt typene og operasjoner rundt forskjellige typer. Dette gjør det egentlig dårlig egnet til webserver.

Målet med backended er å komunisere med databasen og legge ut endepunkter som frontenden min kan kommunisere med.

Det som skiller Rust fra andre programmeringsspråk er at det er veldig mye innebygget minnesikkerhet, dette oppnår Rust med sin minnemanager og unike "eierskap" konsept.
Backenden min er skrevet i Rust. Den består av en API som hostes med actix web rammeverket. 

#### Kode
Rust fungerer som C++ og C hvor det trenger en main funksjon som inneholder koden du ønsker å kjøre. Rust er annerledes der det kjører fra en main.rs fil. Du kan utvide med flere filer med mod.rs filer og mapper.

Viktige filer fra backenden min er:
- main.rs - All koden du ønsker å kjøre når programmer starter må ligge her
- Cargo.toml - Spesifikasjoner for avhengiheter og hvordan exeen skal se ut
- .env - Environment variabler 



>curl -X POST -H "Content-Type: application/json" -d "{\"username\":\"SuperGamer\", \"email\":\"your_email@example.com\", \"passwordhash\":\"your_password\", \"user_timestamp\":\"2024-05-10T09:22:08Z\", \"admin\":false, \"banned\":false}" http://localhost:7175/api/register