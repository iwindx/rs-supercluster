const { SuperCluster } = require("../index");
const Supercluster1 = require('supercluster');
const points = [{
    type: 'Feature',
    properties: {
        id: '1'
    },
    geometry: {
        type: 'Point',
        coordinates: [116.40, 39.92]
    }
}]
const supercluster1 = new Supercluster1();
supercluster1.load(points);

const superCluster = new SuperCluster();

superCluster.load(points);
// console.log('superCluster', superCluster);



