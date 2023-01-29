# Walking Skeleton Addon

## Modo de uso
1. Correr 
Vscode dentro docker
- Instalar extensión `Remote development`
- Agregar usuario a grupo docker `sudo usermod -aG docker $USER`
- Abrir vscode, hacer ctrl + shift + p y click en `Dev container: Reopen in container`
- Esperar ...

o Docker por separado 
```
sudo docker compose run -p 8888:8888 web
```

Dentro de docker
```
# Correr todo el proyecto
make all

# Abrir notebook jupyter
jupyter notebook --allow-root --ip 0.0.0.0
```

2. Cargar como extension temporal a firefox

## Requisitos
Tener instalado [Anaconda](https://www.anaconda.com/)

## Estructura del proyecto 
- Classifier: Carpeta con el notebook de la red neuronal
- src: Codigo rust
- www: codigo javascript que une el clasificador y carga tensorflowjs
- manifest.js: Archivo a cargar a firefox

## Threads
### Errores
- Error de seguridad: El contenido en  no puede cargar o enlazar a moz-extension://df6bbb67-d022-4c3b-b871-44cd39274eea/pkg/wasmaddon.js.
- Content Security Policy: La configuración de la página bloqueó la carga de un recurso en blob:https://www.mozilla.org/03858cb4-3808-4777-b126-7f39a6c454f4 ("child-src").


### Links
- Wasm-threads: https://github.com/chemicstry/wasm_thread#wasm_thread
- content_security_policy: https://developer.mozilla.org/en-US/docs/Mozilla/Add-ons/WebExtensions/Content_Security_Policy#webassembly

### Para correrlo
rustup toolchain install nightly
rustup component add rust-src --toolchain nightly-x86_64-unknown-linux-gnu
cargo +nightly build --target wasm32-unknown-unknown --release -Z build-std=std,panic_abort
