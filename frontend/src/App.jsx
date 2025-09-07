import { useState } from "react";
import Navbar, { MobileMenu } from "./components/Navbar";
import Contributions from "./components/Contributions";
import { Issues } from "./components/Issues";
import Home from "./components/Home";

const App = () => {
  const [menuOpen, setMenuOpen] = useState(false);

  return (
    <div className="min-h-screen flex flex-col">
      <Navbar menuOpen={menuOpen} setMenuOpen={setMenuOpen} />

      {menuOpen && <MobileMenu menuOpen={menuOpen} setMenuOpen={setMenuOpen} />}

      <main className="flex-1 px-4 md:px-8 mt-16">
        <Home/>
        <Contributions />
        <Issues />
      </main>
    </div>
  );
};

export default App;
