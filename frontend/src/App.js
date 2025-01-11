import logo from './logo.svg';
import './App.css';
import MovieComponent from './components/MovieComponent/MovieComponent';
import PianoSheetComponent from './components/PianoSheetComponent/PianoSheetComponent';

function App() {
  return (
    <div className="App">
      <header className="App-header">
        <img src={logo} className="App-logo" alt="logo" />
        <p>
          Edit <code>src/App.js</code> and save to reload.
        </p>
        <a
          className="App-link"
          href="https://reactjs.org"
          target="_blank"
          rel="noopener noreferrer"
        >
          Learn React
        </a>
        <MovieComponent></MovieComponent>
        <PianoSheetComponent></PianoSheetComponent>
      </header>
    </div>
  );
}

export default App;
