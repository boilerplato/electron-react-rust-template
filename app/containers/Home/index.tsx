import React from 'react';
import { Button } from 'antd';
import './index.css';

type HomeProps = {};

type HomeState = {};

class Home extends React.Component<HomeProps, HomeState> {
  render() {
    return (
      <div className="home">
        Home page
        <Button type="primary">Click me</Button>
      </div>
    );
  }
}

export default Home;
