import React, { useEffect, useState } from 'react';
import { Container, Typography, Box, Paper, LinearProgress } from '@mui/material';

function App() {
  const [data, setData] = useState(null);

  useEffect(() => {
    const fetchData = () => {
      fetch('http://127.0.0.1:8000/api/getdata')
        .then(res => res.json())
        .then(res => {
          if(res.code === 200){
            setData(res.data);
          }
        })
        .catch(err => console.error(err));
    }

    fetchData();
    const interval = setInterval(fetchData, 1000);
    return () => clearInterval(interval);
  }, []);

  if(!data) return <Typography style={{ color: '#fff' }}>Loading...</Typography>;

  return (
    <div style={{ backgroundColor: '#121212', minHeight: '100vh', padding: '20px' }}>
      <Container maxWidth="sm">
        <Paper elevation={3} style={{ padding: '20px', backgroundColor: '#1e1e1e', color: '#fff' }}>
          <Typography variant="h5">Minecraft 服务器在线率监视器</Typography>
          <Box mt={2} mb={2}>
            <Typography>
              当前状态: <span style={{ color: data.color1 }}>{data.current}</span>
            </Typography>
            <Typography>
              在线率: <span style={{ color: data.color2 }}> {data.rate}%</span>
            </Typography>
            <Box mt={1}>
              <LinearProgress
                variant="determinate"
                value={parseFloat(data.rate)}
                style={{
                  height: '10px',
                  borderRadius: '5px',
                  backgroundColor: '#333',
                  '& .MuiLinearProgress-bar': { backgroundColor: data.color2 }
                }}
              />
            </Box>
          </Box>
          <Box mt={2}>
            <Typography variant="subtitle1">详细信息:</Typography>
            <div
              style={{ whiteSpace: 'pre-wrap', wordBreak: 'break-word', color: '#fff' }}
              dangerouslySetInnerHTML={{ __html: data.verboseinfo }}
            />
          </Box>
        </Paper>
      </Container>
    </div>
  );
}

export default App;
