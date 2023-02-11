import '@riotjs/hot-reload'

// Global CSS Imports
import 'normalize.css'
import 'milligram'

import { component } from 'riot'
import App from './app.riot'

import registerGlobalComponents from './register-global-components'

// register
registerGlobalComponents()

// mount the root tag
component(App)(document.getElementById('root'))
