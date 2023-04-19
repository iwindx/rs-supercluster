/* tslint:disable */
/* eslint-disable */

/* auto-generated by NAPI-RS */

export interface DefaultOptions {
  minZoom?: number
  maxZoom?: number
  minPoints?: number
  radius?: number
  extent?: number
  nodeSize?: number
  log?: boolean
  generateId?: boolean
}
export interface Geometry {
  type: string
  coordinates: Array<number>
}
export interface Feature {
  type: string
  properties: Record<string, unknown>
  geometry: Geometry
}
export class SuperCluster {
  constructor(options?: DefaultOptions | undefined | null)
  load(points: Array<Feature>): SuperCluster
}
