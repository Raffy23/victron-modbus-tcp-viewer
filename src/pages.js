export default [
  {
    path: '/',
    label: 'Data Viewer',
    componentName: 'viewer'
  }, {
    path: '/settings',
    label: 'Settings',
    componentName: 'settings',
  }, {
    path: '/connection-error(.*)',
    label: 'Connection Error',
    componentName: 'connection-error'
  }
]
