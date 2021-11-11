import '../../styles/globals.css';
import "antd/dist/antd.css";
import "../assets/styles/main.scss";
import { observer } from 'mobx-react';
import { useEffect } from 'react';
import { AuthStore } from '../stores/AuthStore';
const MyApp = observer(({ Component, pageProps }) => {
  const { tryToConnect } = AuthStore

  useEffect(() => {
    tryToConnect();
  }, [])
  return <Component {...pageProps} />
})

export default MyApp
