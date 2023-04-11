const { SuperCluster } = require("../index");
const Supercluster1 = require('supercluster');

const superCluster = new SuperCluster();
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
superCluster.load(points);
// console.log('superCluster', superCluster);


const supercluster1 = new Supercluster1();
supercluster1.load(points);
