import { open } from '@tauri-apps/api/dialog';
import { invoke } from "@tauri-apps/api/tauri";
import { useEffect, useState } from 'react'

export const Converter = () => {
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
            setFilePath('Please select a file!')
        } else {
            setButtonDisabled(true)
            setButtonText('Processing...')
            await invoke('process_file', {
                filepath: filePath,
                clubname: clubName
            }).then(msg => {
                console.log(msg === "Converted successfully")
                setButtonText(msg)
                setButtonDisabled(false)
            });
        }
    }

    const handleClubChange = (e) => {
        setClubName(e.target.value)
        setButtonText('Convert')
    }

    useEffect(() => {
        function loadPreferences() {
            const preferences = invoke('load_preferences')
            console.log("Pref ", preferences.then(result => {
                setClubName(result)
                console.log('setClubName to: ', result)
            }))
        }

        loadPreferences()
        // console.log("useEffect ran...");
    }, [])

    return (
        <>            
            <div className="card">
                <form onSubmit={call_rust}>
                    <div className="information">
                        <label>*Club Name</label>
                        <input tabIndex={0} type="text" placeholder="Please insert club name" onChange={handleClubChange} value={clubName} required />
                    </div>
                    <div className="information">
                        <label>*{filePath}</label>
                        <button tabIndex={1} onClick={openFileDialog}>Select File</button>
                    </div>
                    <div>
                        <button tabIndex={2} type="submit" disabled={buttonDisabled}>{buttonText}</button>
                    </div>
                </form>
            </div>
        </>
    )
}
