import pdf2csvLogo from './assets/pdf2csv.svg'
import './App.css'
import { open } from '@tauri-apps/api/dialog';
import { invoke } from "@tauri-apps/api/tauri";
import { useState } from 'react'

function App() {

  const [clubName, setClubName] = useState('')
  const [filePath, setFilePath] = useState('No file selected')
  const [buttonText, setButtonText] = useState('Convert')
  const [buttonDisabled, setButtonDisabled] = useState(true)

  // Open a selection dialog for image files
  async function openFileDialog(e) {
    // Prevent the default action from opening the file
    e.preventDefault()
    const filePath = await open({
      multiple: false,
      filters: [{
        name: 'PDF',
        extensions: ['pdf']
      }]
    });
    setFilePath(filePath)
    setButtonText('Convert')
    setButtonDisabled(false)

  }

  async function call_rust(e) {
    e.preventDefault()
    if (filePath === 'No file selected' || filePath === 'Please select a file!') {
      setFilePath('Please select a file!');
    } else {
      setButtonDisabled(true)
      setButtonText('Processing...');
      await invoke('process_file', {
        filepath: filePath,
        clubname: clubName
      }).then(msg => {
        console.log(msg === "Converted successfully");
        setButtonText(msg);
        setButtonDisabled(false);
      });
    }
  }

  const handleClubChange = (e) => {
    setClubName(e.target.value)
    setButtonText('Convert')
  }

  return (
    <div className="App">
      <div>
        <img class="logo" src={pdf2csvLogo} alt="PDF2CSV Logo" />
      </div>
      <h1>PDF2CSV</h1>
      <div className="card">
        <form onSubmit={call_rust}>
          <div className="information">
            <label>Club Name</label>
            <input type="text" placeholder="Please insert club name" onChange={handleClubChange} required />
          </div>
          <div className="information">
            <label>{filePath}</label>
            <button onClick={openFileDialog}>Select File</button>
          </div>
          <div>
            <button type="submit" disabled={buttonDisabled}>{buttonText}</button>
          </div>
        </form>
      </div >
    </div >
  )
}

export default App