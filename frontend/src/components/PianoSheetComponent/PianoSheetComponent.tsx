import { useEffect, useState } from "react";
import classes from './PianoSheetComponent.scss';

const PianoSheetComponent = () => {

   const PIANO_SHEETS_ENDPOINT = 'http://localhost:8080/piano-sheets';

   const [data, setData] = useState<PianoSheet[]>([]);

   useEffect(() => {
      const fetchData = async () => {
         try {
            const response = await fetch(PIANO_SHEETS_ENDPOINT,);
            const result: PianoSheet[] = await response.json();
            console.log("result", result)
            setData(result);
         } catch (error) {
            console.error('Error fetching data; ', error);
         }
      }
      fetchData();
   }, []);


   return (
      <div>
         <h1>Piano Sheets</h1>
         <table className="`${classes.table}`">
         <th>Chord Type</th>
         <th>Exemple</th>
         <th>Formula</th>
         <th>Sound Quality</th>
         {
            data.map((sheet,index) => (
               <tr key="{index}">
                  <td>{sheet.chord_type}</td>
                  <td>{sheet.exemple}</td>
                  <td>{sheet.formula}</td>
                  <td>{sheet.sound_quality}</td>
               </tr>
            ))
         }
         </table>
      </div>
   );
}

export default PianoSheetComponent;
