# Walking Skeleton Addon

## Modo de uso
1. Correr 
Docker 
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

