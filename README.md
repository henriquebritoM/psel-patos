# PATOS PSEL

Linguagem escolhida: Rust <br>
Rust é a linguagem em que eu tenho mais experiência e maior conforto para programar,
embora não seja a mais simples para esse tipo de aplicação, possui todas as ferramentas necessárias
para a implementação. <br>
Pessoalmente, eu gosto mais da ergonomia e sintaxe da linguagem do que dos esteriótipos que geralmente são associados à ela,
como "memory safety" e "💥 blazingly fast", até porque é perfeitamente possível escrever programas que não sejam nem seguros nem rápidos.

# Como usar:

**É necessário ter o cargo instalado** <br>
se não tiver, siga as [instruções de instalação](https://rust-lang.org/tools/install)

- ## Setup: 
    - ### Linux: <br>
        1- Na root do diretório, rode o scrip com 
        ```
            ./start.sh
        ``` 
        
        2- Para parar a execução, digite no mesmo terminal 
        ```cntr + c```

    - ### Windows: <br>
        1- Abra dois terminais na root do diretório
        2- No primeiro, execute o servidor de aplicação 
        ```
        cargo run --release --bin app_server
        ``` 
        3- No segundo, execute o servidor reverse proxy
        ```
        cargo run --release --bin web_server
        ```
        4- Para parar a execução, digite, em ambos terminais
        ```cntr + c``` 

- ## Acessando:
    - No navegador, digite a URL 
    ```
        localhost:8080
    ```

    Acredito que UI está bem didática (e só tem 4 botões), apenas siga seu coração e seja feliz >->

# Sobre o projeto

O projeto possui alguns testes unitários, que garantem (até certo ponto) o funcionamento correto. <br>
Para realizar os testes, vá até a root do repositório e execute o cargo
```
cargo test
``` 

A maioria dos erros é tratado e resulta em um ```BadRequest``` ou ```InternalServerError```,
mas há algumas situações onde o programa pode dar panic!() (acredito que apenas nos erros irrecuperáveis).

Como todo programa em rust, há um memory safety decente, acredito não ter criado dangling pointers nem data races,
todos dados compartilhados entre threads possuem o lifetime ```'static``` e são do tipo read-only.

# Funcionamento

O funcionamento consiste de 3 etapas gerais:

### WEB -> R PROXY
- O client envia uma request para o proxy, dependendo da request, o proxy já envia uma response diretamente (como redirecionamento de requests GET para o path "/") <br>
Se houver algum problema na request, ou se o endpoint nãoo for válido, o proxy também envia diretamente response com o erro correspondente

### R PROXY -> APPLICATION -> R PROXY
- O r proxy encaminha a request para o servidor de aplicação e recebe uma response <br>
Note que não há nenhuma autenticação nesse processo, qualquer request para a porta da aplicação será respondida <br>
Também é possivel enviar para a aplicação uma request diferente da recebida pelo proxy, mas nesse projeto isso não é utilizado 

### R PROXY -> WEB
- O r proxy envia a response recebida pela aplicação para a web

#### OBSERVAÇÕES
- Não foi estipulado um timeout para coneções, ou seja, elas só serão fechadas quando uma das partes (server/client) enviar o header ```Connection: Close```
- A leitura de requests/responses que contenham body depende do header ```Content-Length: ```, não será lido nenhum body se este header não estiver presente
- A porta na qual o r proxy e o app se conectam muda a cada execução, você pode conferir a porta atual em [porta](socket_addr.json)

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
