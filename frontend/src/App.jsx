import { useState } from "react"
import Navbar ,{MobileMenu}from "./components/Navbar"

const App =()=>{
    const [menuOpen,setMenuOpen]=useState(false)
    return(
       <>
        <Navbar menuOpen={menuOpen}setMenuOpen={setMenuOpen}/>
        {menuOpen && <MobileMenu  menuOpen={menuOpen} setMenuOpen={setMenuOpen}/>}
        </>

    )
}

export default App