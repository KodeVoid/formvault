import { useState } from "react";

export const MobileMenu = ({ menuOpen, setMenuOpen }) => {
  if (!menuOpen) return null;
  
  return (
    <div className="fixed inset-0 h-full z-50 bg-black/30 backdrop-blur-lg transition-all duration-300 ease-in-out">
      <button 
        onClick={() => { setMenuOpen(false) }} 
        className="absolute top-6 right-6 text-white text-2xl"
      >
        &times;
      </button>
      <div className="flex flex-col items-center justify-center h-full">
        <ul className="flex flex-col items-center space-y-6 text-white text-xl">
          <li><a href="">Home</a></li>
          <li><a href="">About</a></li>
          <li><a href="">Contribution</a></li>
          <li><a href="">WaitList</a></li>
        </ul>
      </div>
    </div>
  )
}

const Navbar = ({ menuOpen, setMenuOpen }) => {
  return (
    <nav className="fixed top-0 w-full flex justify-between items-center border-b-2 border-white min-h-4 backdrop-blur-lg px-4">
      <div className="ml-5">
        <h1>FormVault</h1>
      </div>
      <ul className="hidden md:flex items-center">
        <li className="mx-5 hover:-translate-y-1 hover:underline transition-transform"><a href="">Home</a></li>
        <li className="mx-5 hover:-translate-y-1 hover:underline transition-transform"><a href="">About</a></li>
        <li className="mx-5 hover:-translate-y-1 hover:underline transition-transform"><a href="">Contribution</a></li>
        <li className="mx-5 hover:-translate-y-1 hover:underline transition-transform"><a href="">WaitList</a></li>
      </ul>
      <div 
        className="text-4xl text-white font-bold mr-5 md:hidden cursor-pointer" 
        onClick={() => { setMenuOpen((prev) => !prev) }}
      >
        <h1>&#9776;</h1>
      </div>
    </nav>
  )
}

export default Navbar;