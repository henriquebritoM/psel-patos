console.log("HELLO");

const formsUpload = document.querySelector('#enviar')
const formsDownload = document.querySelector('#receber')

//  cerca de metade desse cógico foi feito com ajuda de IA
//  não manjo o bastante de js pra fazer isso sozinho não

let files = []
atualizaArquivosDisponiveis()
console.log(files)

async function atualizaArquivosDisponiveis() {
    files = await getFiles();
    console.log(files)

    let arquivos = formsDownload.querySelector("#arquivos")

    arquivos.innerHTML = '';    //  remove todos elementos anteriores
    
    for (let file of files) {
        const option = document.createElement('option');
        option.value = file;
        option.textContent = file;
        arquivos.appendChild(option);
    }

    console.log(files)
}

formsUpload.addEventListener('submit', async function (event) {
    event.preventDefault();
    let input_file = formsUpload.querySelector('#enviar-file')
    let file = input_file.files[0]

    try {
        // Envio
        await sendFile(file);
        alert(`Arquivo '${file.name}' enviado com sucesso!`); // Feedback para o usuário
        
        // Atualiza a lista após o sucesso
        await atualizaArquivosDisponiveis(); 
    } catch (error) {
        console.error("Erro no upload:", error);
        alert(`Falha ao enviar o arquivo: ${error.message}`); // Feedback para o usuário
    }
});

formsDownload.addEventListener('submit', async function (event) {
    event.preventDefault();

    let target = formsDownload.querySelector('#receber-file')
    let file = target.value

    event.target.reset();

    // busca do backend
    let blob = await getFile(file)
    iniciarDownload(blob, file);

});

async function getFiles() { 
    const url = "http://localhost:8080/files";

    const response = await fetch(url);

    if (!response.ok) {
        throw new Error(`HTTP error! Status: ${response.status} (${response.statusText})`);
    }

    const data = await response.json();
    
    return data;
}

async function getFile(fileName) {
    let url = "http://localhost:8080/files/";
    url += fileName;

    const response = await fetch(url);

    return await response.blob();
    
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

function iniciarDownload(blob, nomeArquivo) {
    // 1. Cria um URL temporário para o Blob
    // Este URL é acessível apenas nesta sessão do navegador
    const url = window.URL.createObjectURL(blob);
    
    // 2. Cria um elemento <a> (link de download)
    const a = document.createElement('a');
    a.style.display = 'none'; // Não precisa ser visível
    a.href = url;
    
    // 3. Define o atributo 'download' para forçar o navegador a baixar
    // em vez de navegar para o arquivo. O valor é o nome que o arquivo terá
    a.download = nomeArquivo;
    
    // 4. Adiciona ao DOM e simula o clique para iniciar o download
    document.body.appendChild(a);
    a.click();
    
    // 5. Limpa o DOM e libera o URL do Blob
    // É crucial chamar revokeObjectURL para liberar a memória
    window.URL.revokeObjectURL(url);
    document.body.removeChild(a);
}