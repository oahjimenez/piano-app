import logo from './logo.svg';
import './App.css';
import MovieComponent from './components/MovieComponent/MovieComponent';
import PianoSheetComponent from './components/PianoSheetComponent/PianoSheetComponent';

function App() {
  return (
    <div className="App">
      <header className="App-header">
        <MovieComponent></MovieComponent>
        <PianoSheetComponent></PianoSheetComponent>
      </header>
    </div>
  );
}

export default App;
