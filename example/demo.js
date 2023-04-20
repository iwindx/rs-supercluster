const { SuperCluster } = require("../index");
const Supercluster1 = require('supercluster');
const points = [{
    type: 'Feature',
    properties: {
        id: '1'
    },
    geometry: {
        type: 'Point',
        coordinates: [120.146734, 30.270399]
    }
}]
const supercluster1 = new Supercluster1();
supercluster1.load(points);
const result = supercluster1.getClusters([116.35916, 28.933157, 122.466295, 30.819028], 6);
console.log('result', result)
const superCluster = new SuperCluster();

superCluster.load(points);
superCluster.getClusters([116.35916, 28.933157, 122.466295, 30.819028], 6);
// console.log('superCluster', superCluster);



