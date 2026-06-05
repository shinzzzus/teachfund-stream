'use client';

import { Activity, ArrowRight, Coins, Database, ShieldCheck } from "lucide-react";
import { ConnectWallet } from "@/components/ConnectWallet";
import { project } from "@/lib/project";

const workflow = [
  ["Open", project.functions.create],
  ["Fund", project.functions.fund],
  ["Verify", project.functions.attest],
  ["Release", project.functions.release],
];

export default function Home() {
  return (
    <main className="page">
      <div className="shell">
        <header className="topbar">
          <div className="brand">
            <div className="mark"><Coins size={20} /></div>
            <span>{project.name}</span>
          </div>
          <ConnectWallet />
        </header>

        <section className="hero">
          <div className="heroText">
            <div className="eyebrow">{project.track}</div>
            <h1>{project.name}</h1>
            <p>{project.oneLine}</p>
            <div className="metrics">
              <div className="metric"><strong>7</strong><span>Stellar flows</span></div>
              <div className="metric"><strong>4</strong><span>Soroban actions</span></div>
              <div className="metric"><strong>XLM</strong><span>SAC escrow</span></div>
            </div>
            <div className="actions">
              <button className="primary">Open payment record</button>
              <button className="secondary">View testnet proof</button>
            </div>
          </div>

          <div className="panel">
            <h2>Contract workflow</h2>
            <div className="workflow">
              {workflow.map(([label, fn], index) => (
                <div className="step" key={fn}>
                  <div className="num">{index + 1}</div>
                  <div><strong>{label}</strong><p className="mono">{fn}</p></div>
                </div>
              ))}
            </div>
          </div>
        </section>

        <section className="grid">
          <div className="panel">
            <h2><ShieldCheck size={18} /> Stellar settlement layer</h2>
            <p>{project.integration}</p>
            <div className="stack">
              <span className="chip">Freighter</span>
              <span className="chip">Soroban RPC</span>
              <span className="chip">Rust contract</span>
              <span className="chip">XLM SAC</span>
            </div>
          </div>
          <div className="panel">
            <h2><Database size={18} /> Full-stack state</h2>
            <p>Next.js handles the user surface, Express and Prisma keep local operational records, and Soroban stores the auditable payment state.</p>
            <p className="mono">Contract: {project.contractId}</p>
          </div>
        </section>

        <section className="panel" style={{ marginTop: 18 }}>
          <h2><Activity size={18} /> Target users</h2>
          <p>{project.users}. Former project name: {project.oldName}.</p>
          <p className="mono">RPC: {project.rpcUrl}</p>
        </section>
      </div>
    </main>
  );
}
