import cors from "cors";
import express from "express";
import { z } from "zod";
import { prisma } from "./database/prisma.js";

export const app = express();
app.use(cors());
app.use(express.json());

const recordSchema = z.object({
  externalId: z.string().min(3),
  owner: z.string().min(10),
  target: z.number().int().positive(),
});

app.get("/api/health", (_req, res) => {
  res.json({ ok: true, project: "TeachFund Stream", network: "testnet" });
});

app.get("/api/records", async (_req, res) => {
  const records = await prisma.paymentRecord.findMany({ orderBy: { createdAt: "desc" } });
  res.json({ records });
});

app.post("/api/records", async (req, res, next) => {
  try {
    const input = recordSchema.parse(req.body);
    const record = await prisma.paymentRecord.create({
      data: {
        externalId: input.externalId,
        owner: input.owner,
        target: input.target,
        status: "draft",
        projectName: "TeachFund Stream",
      },
    });
    res.status(201).json({ record });
  } catch (error) {
    next(error);
  }
});

app.get("/api/payments/quote", (_req, res) => {
  res.json({
    accepts: ["x402", "stellar-testnet-xlm-sac"],
    project: "TeachFund Stream",
    network: "testnet",
    assetContract: process.env.XLM_SAC,
    paymentRequirements: {
      scheme: "exact",
      amount: "100000",
      payTo: process.env.TREASURY_PUBLIC_KEY || "SET_TREASURY_PUBLIC_KEY",
      memo: "teachfund-stream-access",
    },
  });
});

app.use((error: unknown, _req: express.Request, res: express.Response, _next: express.NextFunction) => {
  if (error instanceof z.ZodError) {
    return res.status(400).json({ error: "ValidationError", details: error.flatten() });
  }
  const message = error instanceof Error ? error.message : "Unexpected error";
  return res.status(500).json({ error: message });
});
