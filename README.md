# PATOS PSEL

Linguagem escolhida: Rust
Motivo: É a linguagem em que eu uso a mais tempo e me sinto mais a vontade usando 

# Como usar:

**É necessário ter o cargo instalado**

- Linux: <br>
    Na root, digite ```./start.sh``` no terminal. Digite ```cntr + c``` para parar a execução.

- Windows: <br>
    Abra dois terminais, digite ```cargo run --release --bin app_server``` no primeiro e ```cargo run --release --bin web_server``` no outro. 
    Para parar a execução, digite ```cntr + c``` em ambos terminais.

- No navegador, digite a URL ```localhost:8080```

O projeto possui alguns testes básicos, que garantem (até certo ponto) o funcionamento correto. Para realizar os testes, digite ```cargo test``` na root .

A maioria dos erros é tratado e resulta em um ```BadRequest``` ou ```InternalServerError```,
mas há algumas situações onde o programa pode dar panic!() (acredito que apenas nos erros irrecuperáveis).

Como todo programa em rust, há um memory safety decente, sem dangling pointers e sem data races,
todos dados compartilhados entre threads possuem o lifetime ```'static``` e são do tipo read-only.

# Versionamento:

O projeto foi dividido em duas branchs:  **main** e **Async**

- main: na main foi montada do zero ao básico a lógica básica do servidor web, do servidor de aplicação e das
bibliotecas locais, que incluem interface com o TCP, parsing de requests/responses e uma micro-framework de 
um servidor funcional.
    
- async: na async as principais mudanças ocorreram nas bibliotecas locais (embora o resto do código tenha sido
alterado para funcionar com as mudanças). Foi implementado o processamento assíncrono para as crates:
    -   tcp_wrapper
    -   smol_server <br>
a última também foi adaptada para funcionar com multithreading, embora só funcione quando executada pela
runtime multithread da crate Tokio.

# Minhas conclusões:

Foi um projeto muito divertido e desafiador, no começo eu não entendia muito sobre network programming,
mas fui estudando com os conteúdos recomendados e pesquisando, acredito ter aprendido bastantece nesse tempo. <br>
A escolha de rust como linguagem foi a mais natural para mim, já que é a que eu tenho mais facilidade de usar,
e por grande parte do tempo foi uma experiência completamente fluida... até chegar na programação assíncrona.
([live reaction](https://youtu.be/Dj8dTZ7d9LE?si=0yQdxMxwqpedn844&t=55)) <br>
Eu nunca havia programado nada assíncrono antes, então tive uma certa dificuldade até entender direito como
eu deveria programar e as mudanças que isso traria para o código. <br>
Montar o smol_server "do zero" foi bem legal, aprendi um monte de coisas, mas é uma tarefa muito exaustiva 
ficar reinventando a roda (e sua roda geralmente é pior) e eu agradeço as várias bibliotecas exelentes que existem.

# Fontes:

Algumas das fontes que eu usei para pesquisa são:

- [Introdução ao HTTP-Mozilla](https://developer.mozilla.org/en-US/docs/Web/HTTP/Guides/Messages)
- [O livro de rust](https://doc.rust-lang.org/rust-by-example/index.html)
- [IETF](https://www.rfc-editor.org/rfc/rfc7230#section-3)
- [crate http - rust](https://docs.rs/http/latest/src/http/status.rs.html#45)
- [Guia do beej](https://beej.us/guide/bgnet/html/split/what-is-a-socket.html)
- [REST API crash course - vídeo](https://youtu.be/qbLc5a9jdXo?si=fftqe83J0a1Oc_9F)
- [medium - explicação sobre servidores](https://medium.com/@firatmelih/understanding-modern-web-applications-web-pages-to-servers-and-hosting-solutions-35bffc819a01)
- [crate axum - rust](https://docs.rs/axum/latest/axum)
- [crate tokio - rust](https://docs.rs/tokio/latest/tokio/index.html)


# Bibliotecas usadas:

Evitei ao máximo bibliotecas com muita coisas prontas, mas usei algumas básicas, dentre elas, as mais relevantes são:

- [crate tokio - rust](https://docs.rs/tokio/latest/tokio/index.html)
Essa crate é responsável por executar a parte assíncrona e fazer o multithreading

- [crate matchit - rust](https://docs.rs/matchit/latest/matchit/index.html)
Um roteador de URLS

- [crate serde - rust](https://docs-rs.translate.goog/serde/latest/serde)
Serialize e deserialize de json
