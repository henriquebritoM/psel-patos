console.log("HELLO");

const formsUpload = document.querySelector('#enviar')
const formsDownload = document.querySelector('#receber')

files = []

function atualizaArquivosDisponiveis() {
    const files = getFiles();
    console.log(files)
}

formsUpload.addEventListener('submit', function (event) {
    event.preventDefault();

    let input_file = formsUpload.querySelector('#enviar-file')
    let file = input_file.files[0]
    console.log(file)

    // manda para o backend
    sendFile(file)

    if (true) atualizaArquivosDisponiveis();    //  atualiza se deu certo
});

formsDownload.addEventListener('submit', function (event) {
    event.preventDefault();

    target = formsUpload.querySelector('#receber-file')

    // manda para o backend
});

async function getFiles() {
    const url = "http://localhost:8080/files";
    fetch(url)
    .then(response => response.json())
    .then(data => {return data})
}

async function sendFile(file) {
    let url = "http://localhost:8080/files/";
    url += file.name;

    const myHeaders = new Headers();
    myHeaders.append("Content-Type", "application/octet-stream");

    fetch(url, {
        method: "POST",
        body: await file.arrayBuffer(),
    });
    // .then(response => response.json())
    // .then(data => {return data})
}