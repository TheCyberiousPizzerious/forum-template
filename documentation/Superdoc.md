# Dokumentasjon

## Innholdsbetegnelse

## Om Oppgaven
Denne seksjonen er om selve oppgaven [hva](#hva) oppgaven er, [hvordan](#hvordan) jeg utførte oppgaven og [hvorfor](#hvorfor) begrunnelser for hva jeg har gjort.

### Hva 
Denne seksjonen handler om funksjonalitet og hva jeg skal bruke for å oppnå det ønsket resultatet.

#### Hva index
- [Netforum](#netforum)
- [Database](#database)
- [Funksjonalitet](#hva-man-skal-kunne-gjøre)
    - [Bruker](#brukere-skal-kunne)
    - [Administrator](#administratorer-skal-kunne)

#### Netforum
Jeg skal lage et nettforum med det jeg anser som standard nettforum funksjonalitet. Brukerer skal autentiseres med JWT (Json Web Tokens). Passord skal hashes og saltes før de lagrer i databasen.

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
|Database|MongoDB|PostGres|

### Hvordan
Jeg skal gjøre dette med min forståelse av Actix-web og Rust. Bruke det jeg kan om MongoDB, viritualisering og Debian.

### Hvorfor
Grunnen til at jeg vil lage et nettforum er fordi alle netforumene jeg har brukt i det siste er dårlig alle bruker samme layout jeg ikke liker. Jeg liker også teknologiene og ønsker å bruke dem mer og få en bedre forståelse for hvordan det fungerer og måtene man kan bruke dem på.

Grunnen til at jeg bruker vite og react på frontenden min.