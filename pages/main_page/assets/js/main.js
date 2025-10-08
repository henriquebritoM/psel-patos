console.log("HELLO");

const formsUpload = document.querySelector('#enviar')
const formsDownload = document.querySelector('#receber')

files = []

async function atualizaArquivosDisponiveis() {
    const files = await getFiles();

    while (formsDownload.firstChild) {
        formsDownload.removeChild(formsDownload.firstChild);
    }
    
    for (let file of files) {
        const option = document.createElement('option');
        option.value = file;
        option.textContent = file;
        listaArquivos.appendChild(option);
    }

    console.log(files)
}

formsUpload.addEventListener('submit', async function (event) {
    event.preventDefault();

    let input_file = formsUpload.querySelector('#enviar-file')
    let file = input_file.files[0]
    console.log(file)

    // manda para o backend
    await sendFile(file)

    atualizaArquivosDisponiveis();
});

formsDownload.addEventListener('submit', async function (event) {
    event.preventDefault();

    let target = formsDownload.querySelector('#receber-file')
    let file = target.value

    await sendFile(file)
    // manda para o backend
});

async function getFile(fileName) {
    let url = "http://localhost:8080/files/";
    url += fileName;

    const myHeaders = new Headers();
    myHeaders.append("Content-Type", "application/octet-stream");

    try {
        const response = await fetch(url, {
            method: "GET",
            headers: myHeaders 
        });

        if (!response.ok) {
            throw new Error(`HTTP error! status: ${response.status} ao tentar baixar ${fileName}`);
        }

        const fileBlob = await response.blob(); 
        
        return fileBlob; 

    } catch (error) {
        console.error("Erro ao tentar obter o arquivo:", error);
        throw error;
    }
}

async function getFiles() {
    const url = "http://localhost:8080/files";
    
    const response = await fetch(url);
    
    if (!response.ok) {
        throw new Error(`HTTP error! status: ${response.status}`);
    }

    const data = await response.json();
    
    return data;
}

async function sendFile(file) {
    let url = "http://localhost:8080/files/";
    url += file.name;

    const response = await fetch(url, {
        method: "POST",
        body: await file.arrayBuffer(),
        headers: { "Content-Type": "application/octet-stream" }
    });
    
    if (!response.ok) {
        throw new Error(`Upload falhou com status: ${response.status}`);
    }
}