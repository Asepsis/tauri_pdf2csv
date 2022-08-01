import './App.css'
import { Converter } from './components/Converter'
import { Menu } from './components/Menu'
import { Header } from './components/Header'

function App() {

  return (
    <div className="App">
      <Header />
      {/* <Menu /> */}
      <Converter />
    </div>
  )
}

export default App
