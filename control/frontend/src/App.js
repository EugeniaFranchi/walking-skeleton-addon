import logo from './logo.svg';
import axios from 'axios';
import './App.css';
import * as React from 'react';
import Table from '@mui/material/Table';
import TableBody from '@mui/material/TableBody';
import TableCell from '@mui/material/TableCell';
import TableContainer from '@mui/material/TableContainer';
import TableHead from '@mui/material/TableHead';
import TableRow from '@mui/material/TableRow';
import Paper from '@mui/material/Paper';


function App() {
  const [data, setData] = React.useState([]);

  React.useEffect(() => {
    axios.get("http://localhost:5000/counter").then(res => {
      let r = []
      Object.keys(res.data).forEach(function(key, index) {
        let s = { site: key, stego: res.data[key].stego, total: res.data[key].total }
        r.push(s); 
      });
      setData(r)
    }).then(err => {
      console.log(err)
    })
  }, [])

  const BasicTable = () => {
    return (
      <TableContainer component={Paper}>
        <Table sx={{ maxWidth: 500 }} aria-label="simple table">
          <TableHead>
            <TableRow>
              <TableCell>Site</TableCell>
              <TableCell align="right">Stego</TableCell>
              <TableCell align="right">Total</TableCell>
              <TableCell align="right">%</TableCell>
            </TableRow>
          </TableHead>
          <TableBody>
            {data.map((row) => (
              <TableRow
                key={row.site}
                sx={{ '&:last-child td, &:last-child th': { border: 0 } }}
              >
                <TableCell align="right">{row.site}</TableCell>
                <TableCell align="right">{row.stego}</TableCell>
                <TableCell align="right">{row.total}</TableCell>
                <TableCell align="right">{row.stego / row.total}</TableCell>
              </TableRow>
            ))}
          </TableBody>
        </Table>
      </TableContainer>
    );
  }

  return (
    <div>
      <BasicTable/>
    </div>
  );
}

export default App;
