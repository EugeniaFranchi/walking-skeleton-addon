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
import Image from './imgs/main_background.jpg';
import { Box } from '@mui/material';


function App() {
  const [data, setData] = React.useState([]);

  React.useEffect(() => {
    axios.get("http://localhost:5000/counter").then(res => {
      let r = []
      Object.keys(res.data).forEach(function (key, index) {
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
      <Box sx={{ backgroundImage: `url(${Image})`, height: "100vh", backgroundSize: "cover", display: 'flex', alignItems: 'center' }}>
        <TableContainer style={{ maxWidth: '100vh', margin: 'auto' }} component={Paper}>
          <Table aria-label="simple table">
            <TableHead>
              <TableRow>
                <TableCell><b>Site</b></TableCell>
                <TableCell align="right"><b>Stego</b></TableCell>
                <TableCell align="right"><b>Total</b></TableCell>
                <TableCell align="right"><b>%</b></TableCell>
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
      </Box>

    );
  }

  return (
    <div>
      <BasicTable />
    </div>
  );
}

export default App;
