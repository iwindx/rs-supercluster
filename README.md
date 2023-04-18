# rs-supercluster

A very fast Napi rs JavaScript extension library for geospatial point clustering of browsers and nodes.

```js
// import as a ES module
import Supercluster from 'supercluster';

// or require in Node / Browserify
const Supercluster = require('supercluster');

const index = new Supercluster({
    radius: 40,
    maxZoom: 16
});
index.load(points);
```

## Methods

#### `load(points)`

Loads an array of [GeoJSON Feature](https://tools.ietf.org/html/rfc7946#section-3.2) objects. Each feature's `geometry` must be a [GeoJSON Point](https://tools.ietf.org/html/rfc7946#section-3.1.2). Once loaded, index is immutable.


## Options

| Option     | Default | Description                                                       |
|------------|---------|-------------------------------------------------------------------|
| minZoom    | 0       | Minimum zoom level at which clusters are generated.               |
| maxZoom    | 16      | Maximum zoom level at which clusters are generated.               |
| minPoints  | 2       | Minimum number of points to form a cluster.                       |
| radius     | 40      | Cluster radius, in pixels.                                        |
| extent     | 512     | (Tiles) Tile extent. Radius is calculated relative to this value. |
| nodeSize   | 64      | Size of the KD-tree leaf node. Affects performance.               |
| log        | false   | Whether timing info should be logged.                             |
| generateId | false   | Whether to generate ids for input features in vector tiles.       |


```
npm run build
npm example/demo.js 

```