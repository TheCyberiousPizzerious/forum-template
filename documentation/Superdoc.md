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

## Lovverk

### GDPR
EU regulasjoner anngående personlig informasjon, gir deg mer kontroll over informasjon som har med deg å gjøre. Det er mye å ta hensyn til, du kan møte på fæle bøter om du ikke er forsiktig etter som du er ansvarlig for sikkerheten og om noen får noen form for skade som konsekvens av at informasjon fra din kilde er lekket får du bot.

Du må kunne gi en bruker all informasjon du har om dem og de må ha muligheten til å slette under artikkel 6, 12, 15 og 17. Brukeren må si ja til å bruke dataen din til markedsføring og ting ikke relatert til tjenesten.

All data som lagres må bli "Pseudonymsated" altså sånn det er lagret kan det ikke bli lenket til et data subjekt.

Du kan få mange store bøter for å bryte de forskjellige GDPR reglene. Det starter som oftest med advarsel om det er alvorlig kan du få forbud mot å behandle data. Ellers er det mye bøter.

### personopplysningsloven
Personopplysningsloven er en norsk lov med formål å beskytte den enkelte mot at personvernet blir krenket gjennom behandling av personopplysninger. Loven gjennomfører EUs personvernforordning i norsk rett. Forordningen er dermed en del av personopplysningsloven og gjelder som norsk lov. 

### Universell utforming (Uu)
Universell utforming er viktig for at alle skal kunne bruke siden din. Det er bare påbudt for statlige og private sider. Når private sider er rettet mot allmenheten er når de må følge kravene om universell utforming. Man skal fylle 47 av 78 kriterier. Det inkluder er noen veldig strenge og vanskelige krav som:
1.4.10 Dynamisk tilpasning, Innhold skal endres til 400% størrelse uten tap av informasjon eller funksjonalitet
1.3.3 Meningsfylt rekefølge, Presenter innhold i en meningsfull rekkefølge
1.3.1 Informasjon og relasjoner, Ting skal være kodet sånn det ser ut som
1.3.4 Visningsretning, Brukeren må få velge om innholdet skal vises i liggende eller stående retning
1.4.4 Endring av tekststørrelse, Tekst kan bli endret til 200 % størrelse uten tap av innhold eller funksjon
2.4.3
Peronsolig tenker jeg at disse er vage og vanskelig å få til om dette ikke er jobben din. 

Konsekvensene for å ikke følge Uu reglene er merkelige det kan være daglige bøter, engangsbøter, IKT-tilsynet git veiledning og annet. 

## Risiko og tiltak

Det er mange risikoer og problemer som kommer med å hoste og drive nettforum. Her er noen jeg tenker er realistiske, det er vannskelig å lage et risikodiagram i markdown så vi tar det punkt for punkt eller risiko for risiko.

### Risikoer og deres respiktive tiltak

#### Strømbrudd
##### Hva skjer
Alles store frykt er strømbrudd, om bruddet skjer ved en av kraftstasjonene er det nesten ingenting annet å gjøre enn å vente på at strømmen kommer tilbake. Strømbrudd kan også skyldes at du drar for mye strøm og at en sikring går.

##### Hvorfor er dette en risiko
Om du ikke har flere serverlokasjoner og din eneste lokasjon blir tjenestene dine utilgjengelig så lenge strømmen er av. Om du driver noen nødvendige services for andre sider kan det ha dårlige konsekvenser for andres servicer samtidig som dine. Hardware har som oftest godt av å være på kontinuelig og det å "dead starte" servere tar ofte lang tid.

##### Tiltak
For å unngå strømbrudd, er det flere proaktive tiltak man kan iverksette. Det er viktig å vedlikeholde og oppgradere det elektriske anlegget regelmessig for å sikre at det tåler belastningen det utsettes for. Installasjon av overspenningsvern kan beskytte mot skader fra lynnedslag eller strømstøt. Videre kan man investere i et nødaggregat som sikkerhetskopi ved strømutfall, spesielt for kritiske systemer eller i områder hvor strømbrudd er vanligere.

#### Ikke ordentlig sikring på siden

##### Hva skjer
Uønskede brukere og aktører med dårlige intensjoner er den største frykten til hvilken som helst online tjeneste. Om siden din ikke har ordentlig protokoller og sikringer for å passe på at uønskede brukere ikke får tilgang til sider du ikke ønsker å gi dem tilgang til kan være veldig skadelig for siden din. De kan også få tilgang til hemmeligheter som databasepassord, informasjon om brukere og tilgang til administrative sider på nettstedet.

##### Hvorfor er dette en risiko
Når en uønsket aktør får tilgang til et nettsted med dårlig autentisering og sikkerhet, kan konsekvensene være alvorlige. For det første kan sensitiv informasjon som brukerdata, finansielle detaljer og personlig identifiserbar informasjon bli kompromittert, noe som kan føre til identitetstyveri og økonomisk svindel. Videre kan angripere utnytte tilgangen til å spre malware eller utføre phishing-angrep mot nettstedets brukere. Dette kan skade nettstedets omdømme og føre til tap av tillit blant brukere og kunder. I tillegg kan dårlig sikkerhet tillate angripere å manipulere nettstedets innhold eller funksjonalitet, noe som kan resultere i juridiske konsekvenser og potensielle bøter for brudd på databeskyttelseslover. Derfor er robust autentisering og sikkerhetspraksis essensielt for å beskytte både nettstedet og dets brukere mot slike risikoer.

##### Tiltak
Det er mange tiltak man kan gjøre for å passe på at sikkerhet på siden er tipp topp. Ting man kan gjøre er å passe på at alle sider som vanlig brukere ikke skal ha tilgang til er utilgjengelig for dem. Man kan også iverksette 2fa sånn at selv om administratorere blir komprimert har man fortsatt et lag med sikkerhet.

#### DDOS
##### Hva skjer
Distributed Denial of Service (DDoS) angrep er en ondsinnet forsøk på å forstyrre normal trafikk av et målrettet nettverk ved å oversvømme det med en flom av Internett-trafikk. Dette oppnås ved å bruke flere kompromitterte datamaskinsystemer som kilder til trafikkoverbelastning. Angriperne utnytter en rekke enheter, inkludert PCer og IoT-enheter, for å lansere et massivt angrep som kan gjøre nettverkstjenester utilgjengelige.

##### Hvorfor er dette en risiko
DDoS-angrep representerer en betydelig risiko fordi de kan ta ned nettsteder eller nettverkstjenester for lengre perioder.

##### Tiltak
For å beskytte mot DDoS-angrep, kan du implementere avanserte sikkerhetsløsninger som inkluderer nettverksovervåking og -analyse for å oppdage og avverge angrep i sanntid. Man kan ta den dyre ruten og bare rate limite og samtidig ha en gigantisk loadbalancer.

#### Phising
##### Hva skjer
Phishing er en teknikk brukt av cyberkriminelle for å lure individer til å avsløre sensitiv informasjon, som brukernavn, passord og kredittkortdetaljer, ved å etterligne en legitim organisasjon i elektronisk kommunikasjon. Ofte involverer dette å sende e-post som ser ut til å være fra velkjente selskaper og ber om at mottakeren klikker på en lenke og oppgir personlig informasjon.

##### Hvorfor er dette en risiko
Phishing er en risiko fordi det direkte utnytter menneskelige svakheter og kan være vanskelig å oppdage selv for forsiktige brukere. Suksessfulle phishing-angrep kan føre til at uønskede kriminelle får tilgang til nettstedet, databaser eller 

##### Tiltak
For å redusere risikoen for phishing, bør organisasjoner implementere sterke e-postfiltreringssystemer og kontinuerlig utdanne ansatte om hvordan de kan gjenkjenne og unngå phishing-forsøk. Det er også viktig å bruke multifaktorautentisering som et ekstra lag med sikkerhet for å beskytte mot uautorisert tilgang.

#### Deling av ulovlige matrialer
##### Hva skjer
Deling av ulovlige materialer refererer til distribusjon eller tilgjengeliggjøring av beskyttet innhold uten tillatelse fra rettighetshaveren. Dette kan inkludere opphavsrettsbeskyttet musikk, filmer, programvare eller andre former for intellektuell eiendom.

##### Hvorfor er dette en risiko
Ulovlig deling av materiale kan utsette en organisasjon for juridiske konsekvenser, inkludert bøter og søksmål. Det kan også skade organisasjonens rykte og forhold til partnere og kunder.

##### Tiltak
For å forhindre ulovlig deling av materialer, bør organisasjoner implementere strenge policyer og tekniske løsninger for å overvåke eller å få brukere til å rapportere andre.

#### Tilgang til hemmeligheter
##### Hva skjer
Tilgang til hemmeligheter innebærer uautorisert oppdagelse, tilgang eller bruk av konfidensiell informasjon som passord, API-nøkler, sertifikater og andre sensitive data som er avgjørende for sikkerheten til en organisasjons IT-infrastruktur.

##### Hvorfor er dette en risiko
Uautorisert tilgang til hemmeligheter kan føre til alvorlige sikkerhetsbrudd, inkludert datalekkasjer og kompromittering av systemer. Det kan også gi angripere muligheten til å manipulere systemer og data, noe som kan ha store konsekvenser.

##### Tiltak
For å beskytte mot uautorisert tilgang til hemmeligheter, bør organisasjoner bruke hemmelighetsstyringsverktøy og -praksis, som inkluderer kryptering, regelmessig rotasjon av legitimasjoner og streng tilgangskontroll. Det er også viktig å overvåke og loggføre tilgang til sensitiv informasjon.

#### Netverksproblemer
##### Hva skjer
Netverksproblemer kan oppstå i mange former, inkludert avbrudd, overbelastning, treg ytelse og konfigurasjonsfeil. Disse problemene kan være forårsaket av teknisk svikt, menneskelige feil, eller ondsinnede angrep. Når nettverksproblemer oppstår, kan det føre til at hele systemer blir utilgjengelige eller at dataoverføring blir betydelig forsinket.

##### Hvorfor er dette en risiko
Nettverksproblemer er en risiko fordi de kan forstyrre den daglige driften av en organisasjon. Dette kan føre til tap av produktivitet, hindre kommunikasjon og i verste fall resultere i tap av viktige data. For bedrifter som er avhengige av online transaksjoner, kan nettverksproblemer også føre til tap av inntekter og kundetillit.

##### Tiltak
For å håndtere nettverksproblemer effektivt, bør organisasjoner investere i robuste nettverksinfrastrukturer med tilstrekkelig redundans og feiltoleranse. Det er også viktig å ha et team av IT-spesialister klare til å diagnostisere og løse problemer raskt. Regelmessig vedlikehold og oppdatering av nettverksutstyr kan også bidra til å forhindre potensielle problemer.

#### Miljø utfordringer
##### Hva skjer
Miljøutfordringer refererer til eksterne hendelser som naturkatastrofer, ekstremvær, klimaendringer eller miljøforurensning som kan påvirke en organisasjons fysiske og digitale infrastruktur. Disse hendelsene kan være uforutsigbare og ha en bred rekkevidde av konsekvenser.

##### Hvorfor er dette en risiko
Miljøutfordringer utgjør en risiko fordi de kan forårsake direkte skade på bygninger, utstyr og infrastruktur. De kan også føre til lengre perioder med driftsstans og i noen tilfeller, fare for liv og helse. I tillegg kan de ha en langvarig innvirkning på en organisasjons økonomiske stabilitet og omdømme.

##### Tiltak
For å redusere risikoen knyttet til miljøutfordringer, bør organisasjoner utvikle omfattende beredskapsplaner som inkluderer evakuering, sikkerhetskopiering av data og gjenopprettingsstrategier. Det er også viktig å ha forsikring som dekker slike hendelser og å investere i bærekraftige praksiser for å bidra til å redusere miljøpåvirkningen.

#### Innbrudd
##### Hva skjer
Innbrudd er at noen får uautorisert fysisk tilgang til server eller jobb lokaler. Fysiske innbrudd kan innebære tyveri eller vandalisme.

##### Hvorfor er dette en risiko
Innbrudd er en risiko fordi det kan føre til tap av verdifulle eiendeler, kompromittering av sensitiv informasjon og potensiell skade på organisasjonens infrastruktur.

##### Tiltak
For å forhindre innbrudd, bør organisasjoner implementere sikkerhetstiltak som inkluderer fysisk sikkerhet som låser og alarmer, samt digital sikkerhet som brannmurer og inntrengningsdeteksjonssystemer. Regelmessige sikkerhetsvurderinger og oppdateringer av sikkerhetsprotokoller er også viktige for å holde tritt med skiftende trusler.

#### Hardware problemer
##### Hva skjer
Hardwareproblemer refererer til fysisk svikt eller feil i datamaskiner, servere, nettverksenheter eller annet IT-utstyr. Dette kan inkludere alt fra harddiskkrasj til feil i serverkjølingssystemer.

##### Hvorfor er dette en risiko
Hardwareproblemer er en risiko fordi de kan føre til uventet nedetid, tap av data og forstyrrelser i tjenester. Dette kan ha en direkte innvirkning på organisasjonens evne til å operere effektivt og levere tjenester til sine kunder.

##### Tiltak
For å minimere effekten av hardwareproblemer, bør organisasjoner ha en vedlikeholdsplan som inkluderer regelmessige inspeksjoner og utskifting av utstyr. Det er også viktig å ha en beredskapsplan som inkluderer backup-systemer og rask gjenoppretting etter svikt.

#### Programmvare utfordringer
##### Hva skjer
Programvareutfordringer kan omfatte bugs, inkompatibilitet mellom systemer, utdatert programvare og sikkerhetssårbarheter. Disse problemene kan føre til feil i systemene, redusert ytelse og potensielle sikkerhetsrisikoer.

##### Hvorfor er dette en risiko
Programvareutfordringer er en risiko fordi de kan kompromittere organisasjonens data og systemer, føre til tap av funksjonalitet og i noen tilfeller utsette organisasjonen for cyberangrep.

##### Tiltak
For å håndtere programvareutfordringer, bør organisasjoner sørge for at all programvare holdes oppdatert med de nyeste sikkerhetsoppdateringene og lappene. Det er også viktig å gjennomføre regelmessig testing og kvalitetssikring for å identifisere og rette opp i eventuelle problemer før de blir kritiske.

## Egenevaluering
Her skal jeg gjøre noe jeg liker som er å reflektere
### Utfordringer jeg har møtt

#### Sette opp Tailwind Css
En av de verste opplevelsene mine har vært å sette opp Tailwind Css. Jeg måtte endre på noen av boilerplate filene som styrer tailwind css.

#### Navigation eller Navigation()
Jeg skulle bruke navigation() for å flytte rundt på sidene fordi Viktor sa det var raskere, men jeg fikk det ikke til å virke fordi jeg skrev Navigation()

#### Hoste React
I vite.config.ts står det standard ip og port som React skal kjøre på men det virket ikke for meg så jeg måtte sette det opp manuelt og spesifisere port og ip.

#### Cors problemer
Jeg hadde glemt å sette opp en CORS policy på backenden min så frontenden min ikke kunne nå den. Det løste seg etter som at det var en crate jeg kunne bruke for å sette opp CORS på en actix web webserver.

#### Mongodb connection
Jeg hadde noen problemer med å sette opp MongoDB serveren min som jeg kjører selv. Problemene mine var tilkoblings problemer hvor jeg måtte åpne porter for MongoDB. Jeg måtte gjøre feilsøking der jeg eliminerte hva som kunne være problemet til det virket.

#### Valg av prosjekt
Jeg hadde mange problemer med å velge prosjektet mitt, jeg hadde nemlig planlagt å sammarbeide om en oppgave med en annen. Det ble dessverre ikke noe av etter som det ble problemer. Vi to slet også med å velge en oppgave så jeg begynte ikke å jobbe før siste halvdel av oppgaven.

Jeg har brukt Actix web litt tidligere, så jeg ville prøve å lage noe med det. Det viste seg at API med Rust ikke er noe serlig.

### Hva har jeg lært

### Hvordan vurderer du selv arbeidet du har gjort

### Hva burde være gjort annerledes

## Curl kommandoer
Om frontenden min ikke snakker ordentlig sammen med backenden min så er dette noen Curl commandoer du kan bruke for å kontakte apien min alikevel. 


Arial 11ptk eller times new roman 12 pkt linjheavstand1.5

>curl -X POST -H "Content-Type: application/json" -d "{\"username\":\"SuperGamer\", \"email\":\"your_email@example.com\", \"passwordhash\":\"your_password\", \"user_timestamp\":\"2024-05-10T09:22:08Z\", \"admin\":false, \"banned\":false}" http://localhost:7175/api/register

## Kilder
https://www.uutilsynet.no/veiledning/losningsforslag-krav/1366
https://en.wikipedia.org/wiki/General_Data_Protection_Regulation#Timeline