# Walking Skeleton Addon

## Acerca del Proyecto

Addon Firefox de estegoanalisis

## Bibliotecas Preinstaladas
- [Anaconda](https://www.anaconda.com/)
- Tensorflow
```
pip install tensorflow
``` 

## Preparación 

### Clasificador
1. Correr notebook (Run All) `classifier.ipyn`
2. Convertir model.h5 a model.json
```
cd extension/model/

tensorflowjs_converter --input_format keras ./model.h5 ./output/
```

### Extension
1. Ir a carpeta webpack 
```
cd extension/webpack
```
2. Instalación de dependencias de node
```
npm install
```

3. Transpilación de index.js+bibliotecas a main.js (debe ser ejecutado cada vez que se modifica index.js)
```
npx webpack
```

4. Cargar el addon en Firefox
- Abrir Firefox
- ir a `about:debugging`
- click en Load Temporary Addon
- seleccionar `extension/manifest.json`

## Uso

Ir a `https://www.mozilla.org/en-US/` y ver la consola (F12) 

