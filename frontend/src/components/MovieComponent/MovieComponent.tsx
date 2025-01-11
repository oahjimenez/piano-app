import React, { useState, useEffect } from 'react';

const MovieComponent = () => {

   const MOVIE_ENDPOINT = 'http://localhost:8080/movies';

   const [data, setData] = useState([]);

   useEffect(() => {
      const fetchData = async () => {
         try {
            const response = await fetch(MOVIE_ENDPOINT,);
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
};

export default MovieComponent;
