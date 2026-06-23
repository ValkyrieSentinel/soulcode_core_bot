import React, { useState, useEffect } from 'react';
import { ethers } from 'ethers';

// ЗАМІНИ НА РЕАЛЬНІ ДАНІ
const CONTRACT_ADDRESS = "0x..."; // Адреса твого задеплоєного Yul-контракту
const ADMIN_WALLET = "0x...";     // Куди падають гроші за підписку

function App() {
  const [userAddress, setUserAddress] = useState("");
  const [isPro, setIsPro] = useState(false);
  const [data, setData] = useState(""); 

  
  const connect = async () => {
    const provider = new ethers.BrowserProvider(window.ethereum);
    const accounts = await provider.send("eth_requestAccounts", []);
    setUserAddress(accounts[0]);
  };

  
  const buySubscription = async () => {
    const provider = new ethers.BrowserProvider(window.ethereum);
    const signer = await provider.getSigner();
    const tx = await signer.sendTransaction({
      to: ADMIN_WALLET,
      value: ethers.parseEther("0.05") 
    });
    await tx.wait();
    setIsPro(true);
    alert("Підписку активовано!");
  };

  
  const runAirdrop = async () => {
    if (!isPro) return alert("Спочатку купи Pro!");
    
    
    const lines = data.trim().split('\n');
    const addresses = lines.map(l => l.split(',')[0].trim());
    const amounts = lines.map(l => ethers.parseEther(l.split(',')[1].trim()));
    const totalAmount = amounts.reduce((a, b) => a + b, 0n);

    
    const res = await fetch('/api/encode', {
      method: 'POST',
      body: JSON.stringify({ addresses, amounts })
    });
    const { calldata } = await res.json();

    
    const fee = totalAmount / 1000n; 
    
    const provider = new ethers.BrowserProvider(window.ethereum);
    const signer = await provider.getSigner();

    
    const tx = await signer.sendTransaction({
      to: CONTRACT_ADDRESS,
      data: calldata,
      value: totalAmount + fee 
    });
    
    alert("Розсилка запущена! Хеш: " + tx.hash);
  };

  return (
    <div className="app-container">
      {!userAddress ? (
        <button onClick={connect}>Підключити гаманець</button>
      ) : (
        <>
          <p>Гаманець: {userAddress.slice(0,6)}...</p>
          
          {!isPro ? (
            <button onClick={buySubscription}>Купити Pro (0.05 ETH)</button>
          ) : (
            <div className="airdrop-panel">
              <textarea 
                placeholder="0xадреса,сума (кожен з нового рядка)" 
                onChange={(e) => setData(e.target.value)}
              />
              <button onClick={runAirdrop}>Запустити масову розсилку</button>
            </div>
          )}
        </>
      )}
    </div>
  );
}

export default App;