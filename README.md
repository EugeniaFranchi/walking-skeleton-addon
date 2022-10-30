# Walking Skeleton Addon

## Acerca del Proyecto

Addon Firefox de estegoanalisis

## Preparación 
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

5. Ir a `https://www.mozilla.org/en-US/` y ver la consola (F12) 


## Uso
