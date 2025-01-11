import { useEffect, useState } from "react";

const PianoSheetComponent = () => {

      const PIANO_SHEETS_ENDPOINT = 'http://localhost:8080/piano-sheets';
   
      const [data, setData] = useState([]);
   
      useEffect(() => {
         const fetchData = async () => {
            try {
               const response = await fetch(PIANO_SHEETS_ENDPOINT,);
               const result = await response.json();
               console.log("result",result)
               setData(result);
            } catch (error) {
               console.error('Error fetching data; ', error);
            }
         }
         fetchData();
      }, []);
   
   
      return (
         <div>
            <h1>API Data</h1>
            <p>{JSON.stringify(data)}</p>
       </div>
      );
}

export default PianoSheetComponent;
