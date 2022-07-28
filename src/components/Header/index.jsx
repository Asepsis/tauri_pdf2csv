import pdf2csvLogo from '../../assets/pdf2csv.svg'

export const Header = () => {
    
    const headerStyle = {
        display: 'flex',
        flexDirection: 'column',
        textAlign: 'center',
        justifyContent: 'center',
        alignItems: 'center',
        // backgroundColor: '#202020',
    }

    return (
        <>
            <div style={headerStyle}>
                <img class="logo" src={pdf2csvLogo} alt="PDF2CSV Logo" />
                <h1>PDF2CSV</h1>
            </div>
            
        </>
    )
}