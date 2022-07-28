
export const Menu = () => {

    const menuStyle = {
        //Hide list type
        listStyle: 'none',
        justifyContent: 'center',
        alignItems: 'center',
    }

    const liStyle = {
        width: '6em',
        display: 'flex',
        justifyContent: 'center',
        alignItems: 'center',
        backgroundColor: '#202020',
        padding: '10px',
        margin: '5px',
        borderRadius: '5px',
        color: '#fff',
        fontSize: '1.0em',
        fontWeight: 'bold',
        cursor: 'pointer',
        transition: 'all 0.3s ease-in-out',
        '&:hover': {
            backgroundColor: '#303030',
            color: '#fff',
            transform: 'translateY(-5px)',
            transition: 'all 0.3s ease-in-out',
        }
    }

    const ulStyle = {
        display: 'flex',
        flexDirection: 'row',
        justifyContent: 'center',
        alignItems: 'center',
        // backgroundColor: '#202020',
        padding: '5px',
        margin: '5px',
        borderRadius: '5px',
        color: '#fff',
        fontSize: '1.0em',
        fontWeight: 'bold',
        cursor: 'pointer',
        transition: 'all 0.3s ease-in-out',
        '&:hover': {
            backgroundColor: '#303030',
            color: '#fff',
            transform: 'translateY(-5px)',
            transition: 'all 0.3s ease-in-out',
        }
    }
    


    return (
        <>
            <div style={menuStyle}>
                <nav>
                    <ul style={ulStyle}>
                        <li style={liStyle}><a href="#">Settings</a></li>
                        <li style={liStyle}><a href="#">Converter</a></li>
                    </ul>
                </nav>
            </div>
        </>
    )
}